const focusDirective = {
  mounted(el, binding) {

    const arg = binding.arg;
    const value = binding.value;

    el.addEventListener("focus", () => {
      if (arg == "color") {
        el.style.color = value;
      }
    });

    el.addEventListener("blur", () => {
      if (arg == "color") el.style.color = "";
    });

    el.focus();
  }
};

export default focusDirective;
