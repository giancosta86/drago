import { expect } from "vitest";
import { RandomParamsError } from "@giancosta86/drago";

expect.extend({
  toThrowRandomParamsError(received, expectedError: RandomParamsError) {
    const { isNot } = this;
    const { printExpected, printReceived } = this.utils;

    const code = received;

    let pass: boolean;
    let actualError: RandomParamsError | null = null;

    try {
      code();
      pass = false;
    } catch (e) {
      actualError = e as RandomParamsError;
      pass = this.equals(actualError, expectedError);
    }

    return {
      pass,
      message: () =>
        isNot
          ? `This unexpected error was received:\n${printReceived(actualError)}`
          : `Mismatching error:\n${printReceived(actualError)}\nexpected:\n${printExpected(expectedError)}`
    };
  }
});
