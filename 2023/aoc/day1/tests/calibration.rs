#![allow(clippy::unwrap_used, clippy::expect_used)]

use day1::calibration::*;


#[test]
fn test_calibration_value_test_a() {
    // arrange
    let s = "1abc2";
    let expected: Option<i8> = Some(12);

    // act
    let actual = calibration_value(s);

    // assert
    assert_eq!(expected, actual)
}

#[test]
fn test_calibration_value_test_b() {
    // arrange
    let s = "pqr3stu8vwx";
    let expected: Option<i8> = Some(38);

    // act
    let actual = calibration_value(s);

    // assert
    assert_eq!(expected, actual)
}

#[test]
fn test_calibration_value_test_c() {
    // arrange
    let s = "a1b2c3d4e5f";
    let expected: Option<i8> = Some(15);

    // act
    let actual = calibration_value(s);

    // assert
    assert_eq!(expected, actual)
}

#[test]
fn test_calibration_value_test_d() {
    // arrange
    let s = "treb7uchet";
    let expected: Option<i8> = Some(77);

    // act
    let actual = calibration_value(s);

    // assert
    assert_eq!(expected, actual)
}

#[test]
fn test_calibration_value_test_e() {
    // arrange
    let s = "two1nine";
    let expected: Option<i8> = Some(29);

    // act
    let actual = calibration_value(s);

    // assert
    assert_eq!(expected, actual)
}

#[test]
fn test_calibration_value_test_f() {
    // arrange
    let s = "eightwothree";
    let expected: Option<i8> = Some(83);

    // act
    let actual = calibration_value(s);

    // assert
    assert_eq!(expected, actual)
}

#[test]
fn test_calibration_value_test_g() {
    // arrange
    let s = "abcone2threexyz";
    let expected: Option<i8> = Some(13);

    // act
    let actual = calibration_value(s);

    // assert
    assert_eq!(expected, actual)
}

#[test]
fn test_calibration_value_test_h() {
    // arrange
    let s = "xtwone3four";
    let expected: Option<i8> = Some(24);

    // act
    let actual = calibration_value(s);

    // assert
    assert_eq!(expected, actual)
}

#[test]
fn test_calibration_value_test_i() {
    // arrange
    let s = "4nineeightseven2";
    let expected: Option<i8> = Some(42);

    // act
    let actual = calibration_value(s);

    // assert
    assert_eq!(expected, actual)
}

#[test]
fn test_calibration_value_test_j() {
    // arrange
    let s = "7pqrstsixteen";
    let expected: Option<i8> = Some(76);

    // act
    let actual = calibration_value(s);

    // assert
    assert_eq!(expected, actual)
}