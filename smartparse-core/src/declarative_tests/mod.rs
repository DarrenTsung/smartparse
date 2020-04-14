#![cfg(test)]

use include_dir::{include_dir, Dir};
use serde_derive::Deserialize;

mod feature;

use self::feature::Feature;

#[derive(Debug, Deserialize)]
struct DeclarativeTests {
    tests: Vec<DeclarativeTest>,
}

#[derive(Debug, Deserialize)]
struct DeclarativeTest {
    input: String,
    expected_features: Vec<Feature>,
}

#[derive(Debug)]
enum FailReason {
    InputEmpty,
    MismatchedNumberOfExpectedFeatures,
    FeatureDoesntMatch,
}

static TEST_FILES_DIR: Dir = include_dir!("src/declarative_tests/test_files");

#[test]
fn declarative_tests_work() {
    for file in TEST_FILES_DIR.files() {
        let tests: DeclarativeTests =
            toml::from_str(file.contents_utf8().expect("string contents"))
                .expect("test file is valid DeclarativeTests");
        run_tests(tests);
    }
}

fn run_tests(tests: DeclarativeTests) {
    let DeclarativeTests { tests } = tests;

    #[derive(Debug)]
    struct FailedDeclaractiveTest<'a> {
        test_index: usize,
        test: &'a DeclarativeTest,
        reason: FailReason,
        features: Option<Vec<Feature>>,
    }

    let mut failed_tests = vec![];
    for (index, test) in tests.iter().enumerate() {
        let mut add_failed_test = |reason, features| {
            failed_tests.push(FailedDeclaractiveTest {
                test_index: index,
                test,
                reason,
                features,
            });
        };

        if test.input.trim().is_empty() {
            add_failed_test(FailReason::InputEmpty, None);
            continue;
        }

        let features: Vec<Feature> = crate::feature::identify(&test.input)
            .into_iter()
            .map(|f| Feature::from(f))
            .collect::<Vec<_>>();
        if features.len() != test.expected_features.len() {
            add_failed_test(
                FailReason::MismatchedNumberOfExpectedFeatures,
                Some(features),
            );
            continue;
        }

        for (expected, feature) in test.expected_features.iter().zip(features.iter()) {
            if expected != feature {
                add_failed_test(FailReason::FeatureDoesntMatch, Some(features));
                break;
            }
        }
    }

    if !failed_tests.is_empty() {
        // Custom formatting for the error message.
        let mut failed_tests_msg = String::new();
        for FailedDeclaractiveTest {
            test_index,
            test,
            reason,
            features,
        } in failed_tests
        {
            let test_identifier_text = if test.input.is_empty() {
                format!("at index: {}", test_index)
            } else {
                format!("with input: '{}'", test.input)
            };

            let reason_text = match reason {
                FailReason::MismatchedNumberOfExpectedFeatures | FailReason::FeatureDoesntMatch => {
                    format!(
                        "\n\texpected: {}\n\tgot: {}",
                        Feature::line_separated_format(&test.expected_features, 1),
                        Feature::line_separated_format(&features.expect("exists"), 1),
                    )
                }
                r => format!("with reason: {:?}", r),
            };
            failed_tests_msg.push_str(&format!(
                "Failed test {} {}\n",
                test_identifier_text, reason_text
            ));
        }

        println!("{}", failed_tests_msg);
        assert!(false, "One of more tests failed!");
    }
}
