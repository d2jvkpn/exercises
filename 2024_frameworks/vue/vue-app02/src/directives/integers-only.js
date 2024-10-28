const integersOnly = {
  mounted(el, binding) {
    if (binding.modifiers.upper) {
      el.value = el.value.toUpperCase();
      el.dispatchEvent(new Event("input"));
    }

    el.addEventListener("keydown", event => {
      const numbers = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
      const moves = ["Backspace", "ArrowLeft", "ArrowRight", "Delete", "Tab", "Home", "End"];
      const letters = ["a", "b", "c", "d", "e", "f", "A", "B", "C", "D", "E", "F"];

      let authorized;

      if (binding.modifiers.hexa) {
        authorized = [...numbers, ...letters, ...moves];
      } else {
        authorized = [...numbers, ...moves];
      }

      if (!authorized.includes(event.key)) {
        event.preventDefault();
      }

      if (binding.modifiers.upper) {
        if (letters.includes(event.key)) {
          const start = el.selectionStart;
          const end = el.selectionEnd;
          const text = el.value;

          const newText = text.substring(0, start) + event.key + text.substring(end);
          el.value = newText.toUpperCase();
          el.setSelectionRange(start + 1, start + 1);
          event.preventDefault();
          el.dispatchEvent(new Event("input"));
        }
      }
    });
  },
}

export default integersOnly;
