<script setup>
  import { ref, defineEmits } from "vue"

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

  const eraseField = () => {
    event.target.value = "";
    count.value = "";
    message.value = "";
  };


  const emit = defineEmits(["get"]);

  const call = () => {
    console.log("~~~ called in MyEvents.vue");
    emit("get", "called");
  };
</script>

<template>
  <br><br>
  <button @click="call()">Call</button>
  <br><br>

  <h3>MyCounter Component</h3>
  Reactive variable count:

  <input type="text"
    @keydown="verifyKey()"
    @input="verifyMax(100)"
    @focus="eraseField()"
    v-model="count"
  />

  <br/><br/>
  Entered value: <b>{{count}}</b>

  <br><br>
  Message : <b>{{message}}</b>
</template>