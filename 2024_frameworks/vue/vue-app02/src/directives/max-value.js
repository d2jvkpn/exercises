const treatment = (el, binding) => {
  const max = binding.value || 100; // 100 by default
  const value = el.value || 0; // Value in the field
  const bold = binding.modifiers.bold;

  if (value > max) {
    el.style.color = "red";
    if (bold) {
      el.style.fontWeight = "bold";
      el.style.fontFamily = "arial";
    }
  } else {
    el.style.color = "";
    el.style.fontWeight = ""; // Removal of "bold"
    el.style.fontFamily = ""; // Removal of "arial"
  }
};

const maxValue = {
  mounted(el, binding) {
    treatment(el, binding);
  },
  updated(el, binding) {
    treatment(el, binding);
  },
};

export default maxValue;
