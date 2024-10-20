import { customRef } from 'vue';

const useUpperCase = (initValue) => {
  return customRef((track, trigger) => {
    let value = initValue;

    return {
      get() {
        track();
        return value.toUpperCase();
      },
      set(newValue) {
        console.log("==> useUpperCase set:", newValue);
        value = newValue;
        trigger();
      }
    };
  });
};

export default useUpperCase;
