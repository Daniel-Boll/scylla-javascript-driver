import test from "ava";

import { Uuid } from "../index.js";

test("Create a random UUID", (t) => {
  const uuid = Uuid.randomV4();
  t.is(uuid.toString().length, 36);
});

test("Create a UUID from a string", (t) => {
  const uuid = Uuid.fromString("123e4567-e89b-12d3-a456-426614174000");
  t.is(uuid.toString(), "123e4567-e89b-12d3-a456-426614174000");
});

test("Should error on creating UUID from malformed string", (t) => {
  const error = t.throws(
    () => {
      throw Uuid.fromString("123e4567-e89b-12d3-a456-42661417400");
    },
    { instanceOf: Error },
  );

  if (!(error instanceof Error)) {
    t.fail("error is not an instance of Error");
    return;
  }

  t.is(
    error.message,
    "Failed to parse UUID: invalid group length in group 4: expected 12, found 11",
  );
});

test("Should not error on creating UUID from well-formed string", (t) => {
  t.notThrows(() => {
    return Uuid.fromString("123e4567-e89b-12d3-a456-426614174000");
  });
});
