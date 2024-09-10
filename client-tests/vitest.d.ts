import { RandomParamsError } from "@giancosta86/drago";

interface CustomMatchers<R = unknown> {
  toThrowRandomParamsError: (error: RandomParamsError) => R;
}

declare module "vitest" {
  interface Assertion<T = any> extends CustomMatchers<T> {}
  interface AsymmetricMatchersContaining extends CustomMatchers {}
}
