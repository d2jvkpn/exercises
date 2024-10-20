import { customRef } from "vue";

const formatDate = (date, format) => {
  const options = { year: 'numeric', month: '2-digit', day: '2-digit' };

  if (format == "MM-DD-YYYY") {
    return date.toLocaleDateString('en-US', options).replace(/\//g, '-');
  } else if (format == "DD-MM-YYYY") {
    return date.toLocaleDateString('en-GB', options).replace(/\//g, '-');
  } else if (format == "MM/DD/YYYY") {
    return date.toLocaleDateString('en-US', options);
  } else if (format == "DD/MM/YYYY") {
    return date.toLocaleDateString('en-GB', options);
  }
}

const useFormatDate = (date, format) => {
  return customRef((track, trigger) => {
    let value = date; // value will be the tracked variable
    return {
      get() {
        track(); // track the dependency when the value is read
        return formatDate(value, format);
      },
      set(newValue) {
        value = newValue; // update the value and trigger reactivity
        trigger();
      }
    };
  });
};

export default useFormatDate;