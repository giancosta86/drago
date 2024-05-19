import { readFile } from "node:fs/promises";
import { fileURLToPath } from "node:url";
import { beforeAll, describe, expect, it } from "vitest";
import initDrago, { LogogramGenerator, RandomParams } from "@giancosta86/drago";

type WorkingTestCase = Readonly<{
  kind: string;
  params: RandomParams;
  expectedLogograms: string;
}>;

describe("Logogram generation", () => {
  beforeAll(async () => {
    const wasmUrl = new URL("../../pkg/drago_bg.wasm", import.meta.url);

    const wasmPath = fileURLToPath(wasmUrl);

    const buffer = await readFile(wasmPath);

    await initDrago(buffer);
  });

  describe.each<WorkingTestCase>([
    {
      kind: "integers",
      params: {
        seed: 90,
        variant: "Simplified",
        integerRange: [92, 92],
        deltaTime: false
      },
      expectedLogograms: "九十二"
    },

    {
      kind: "fractions",
      params: {
        seed: 90,
        variant: "Simplified",
        fraction: {
          denominatorRange: [7, 7],
          numeratorRange: [3, 3]
        },
        deltaTime: false
      },
      expectedLogograms: "七分之三"
    },

    {
      kind: "counts",
      params: {
        seed: 90,
        variant: "Simplified",
        countRange: [2, 2],
        deltaTime: false
      },
      expectedLogograms: "两"
    },

    {
      kind: "counts - traditional",
      params: {
        seed: 90,
        variant: "Traditional",
        countRange: [2, 2],
        deltaTime: false
      },
      expectedLogograms: "兩"
    },

    {
      kind: "digit sequences",
      params: {
        seed: 90,
        variant: "Simplified",
        digitSequenceLengthRange: [5, 5],
        deltaTime: false
      },
      expectedLogograms: "七二四二六"
    },

    {
      kind: "decimals",
      params: {
        seed: 90,
        variant: "Simplified",
        decimal: {
          integerRange: [42, 42],
          fractionalLengthRange: [3, 3]
        },
        deltaTime: false
      },
      expectedLogograms: "四十二点四二六"
    },

    {
      kind: "renminbi",
      params: {
        seed: 90,
        variant: "Simplified",
        renminbi: {
          style: "Formal",
          yuanRange: [7, 7],
          includeDimes: true,
          includeCents: true
        },
        deltaTime: false
      },
      expectedLogograms: "七元七角两分"
    },

    {
      kind: "dates",
      params: {
        seed: 90,
        variant: "Simplified",
        date: {
          formal: true,
          pattern: "YearMonthDayWeekDay",
          yearRange: [2019, 2019],
          weekFormat: "XinqQi"
        },
        deltaTime: false
      },
      expectedLogograms: "二零一九年九月九号星期三"
    },

    {
      kind: "linear time",
      params: {
        seed: 90,
        variant: "Simplified",
        linearTime: {
          dayPart: true,
          includeSecond: true
        },
        deltaTime: false
      },
      expectedLogograms: "上午八点四十三分十七秒"
    },

    {
      kind: "delta time",
      params: {
        seed: 90,
        variant: "Simplified",
        deltaTime: true
      },
      expectedLogograms: "六点差十七分"
    }
  ])("for $kind", ({ params, expectedLogograms }) => {
    it("should work", () => {
      const generator = LogogramGenerator.create(params);

      const actualLogograms = generator.logograms();

      expect(actualLogograms).toBe(expectedLogograms);
    });
  });

  describe("for potentially zero-length digit sequence", () => {
    it("should throw", () => {
      expect(() => {
        LogogramGenerator.create({
          seed: 90,
          variant: "Simplified",
          digitSequenceLengthRange: [0, 5],
          deltaTime: false
        });
      }).toThrowRandomParamsError({
        DigitSequenceLength: {
          InvalidStart: 0
        }
      });
    });
  });

  describe("for no random settings", () => {
    it("should throw", () => {
      expect(() => {
        LogogramGenerator.create({
          seed: 90,
          variant: "Simplified",
          deltaTime: false
        });
      }).toThrowRandomParamsError("EmptyRandomParams");
    });
  });
});
