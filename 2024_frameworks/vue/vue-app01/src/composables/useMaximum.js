import { customRef } from "vue";

const useMaximum = (max) => {
  // Create a custom reference (customRef)
  return customRef((track, trigger) => {
    let value = 0; // value will be the variable being tracked, initialized here to 0.

    return {
      get() {
        // Track the dependency when the value is read.
        track();
        console.log("==> get value =", value);
        return value;
      },
      set(newValue) {
        // Update the value and trigger reactivity.
        if (newValue <= max && value != newValue) {
          value = newValue;
          console.log("==> set value =", value);
          trigger();
        }
      }
    };
  });
}

export default useMaximum;