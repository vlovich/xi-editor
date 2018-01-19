// Copyright 2018 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![cfg_attr(feature = "benchmarks", feature(test))]

extern crate xi_trace;

#[cfg(feature = "chrome_trace_event")]
extern crate serde;
#[cfg(feature = "chrome_trace_event")]
#[macro_use]
extern crate serde_json;

#[cfg(all(test, feature = "benchmarks"))]
extern crate test;

#[cfg(feature = "chrome_trace_event")]
pub mod chrome_trace;

#[cfg(test)]
mod tests {
    use super::*;
    use xi_trace;
    #[cfg(feature = "benchmarks")]
    use test::Bencher;

    #[cfg(feature = "chrome_trace_event")]
    #[test]
    fn test_chrome_trace_serialization() {
        use xi_trace::*;

        let trace = Trace::enabled(Config::with_limit_count(10));
        trace.instant("sample1", &["test", "chrome"]);
        trace.instant_payload("sample2", &["test", "chrome"], "payload 2");
        trace.instant_payload("sample3", &["test", "chrome"], "payload 3");
        trace.closure_payload("sample4", &["test", "chrome"],|| (), "payload 4");

        let samples = trace.samples_cloned_unsorted();

        let mut serialized = Vec::<u8>::new();
        let result = chrome_trace::serialize(
            &samples, chrome_trace::OutputFormat::JsonArray, &mut serialized);
        assert!(result.is_ok());

        let decoded_result : Vec<serde_json::Value> = serde_json::from_slice(&serialized).unwrap();
        assert_eq!(decoded_result.len(), 5);
        for i in 0..3 {
            assert_eq!(decoded_result[i]["name"].as_str().unwrap(), samples[i].name);
            assert_eq!(decoded_result[i]["cat"].as_str().unwrap(), "test,chrome");
            assert_eq!(decoded_result[i]["ph"].as_str().unwrap(), "i");
            assert_eq!(decoded_result[i]["ts"], samples[i].start_ns / 1000);
            assert_eq!(decoded_result[i]["args"]["payload"], json!(samples[i].payload));
        }
        assert_eq!(decoded_result[3]["ph"], "B");
        assert_eq!(decoded_result[4]["ph"], "E");
    }

    #[cfg(all(feature = "chrome_trace_event", feature = "benchmarks"))]
    #[bench]
    fn bench_chrome_trace_serialization_one_element(b: &mut Bencher) {
        use chrome_trace::*;

        let mut serialized = Vec::<u8>::new();
        let samples = [xi_trace::Sample::new_instant("trace1", &["benchmark", "test"], None)];
        b.iter(|| {
            serialized.clear();
            serialize(samples.iter(), OutputFormat::JsonArray, &mut serialized).unwrap();
        });
    }

    #[cfg(all(feature = "chrome_trace_event", feature = "benchmarks"))]
    #[bench]
    fn bench_chrome_trace_serialization_multiple_elements(b: &mut Bencher) {
        use chrome_trace::*;
        use xi_trace::*;

        let mut serialized = Vec::<u8>::new();
        let mut samples = [
            Sample::new_instant("trace1", &["benchmark", "test"], None),
            Sample::new_instant("trace2", &["benchmark"], None),
            Sample::new("trace3", &["benchmark"], Some(StrCow::from("some payload"))),
            Sample::new_instant("trace4", &["benchmark"], None)];
        let sample_start = samples[2].start_ns;
        samples[2].set_end_ns(sample_start);

        b.iter(|| {
            serialized.clear();
            serialize(samples.iter(), OutputFormat::JsonArray, &mut serialized).unwrap();
        });
    }
}
