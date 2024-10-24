<script setup>
  import { ref } from "vue"

  const count = ref();

  const verifyKey = () => {
    const numbers = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];

    const moves = [
        "Backspace", "ArrowLeft", "ArrowRight", "Delete", "Tab",
        "Home", "End",
    ];

    let authorized = [...numbers, ...moves];

    if (!authorized.includes(event.key)) {
      // The event object is available here.      
      event.preventDefault();
    }
  };

  const message = ref("");

  const verifyMax = (num) => {
    message.value = "";
    // The event object is available here.
    if (parseInt(event.target.value) > num) {
      message.value = "Don't exceed 100!";
    }
  };
</script>

<template>
  <h3>MyCounter Component</h3>
  Reactive variable count:
  <input type="text" @keydown="verifyKey()" @input="verifyMax(100)" v-model="count" />

  <br/><br/>
  Entered value: <b>{{count}}</b>

  <br><br>
  Message : <b>{{message}}</b>
</template>