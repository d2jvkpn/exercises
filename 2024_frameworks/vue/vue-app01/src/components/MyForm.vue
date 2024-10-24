<script setup>
  import { ref } from 'vue';

  const person = ref({
    name: "",
    birthdate: "",
    maritalStatus: "",
  });

  let dates = [];
  for (let year=2024; year > 1990; year--) {
    dates.push(year);
  }
</script>

<template>
  <h3>Input Form</h3>
  Name: <input type="text" v-model="person.name" />

  <br/><br/>
  Date of Birth:
  <select v-model="person.birthdate" >
  <option v-for="date in dates" :value="date" :key="date">{{date}}</option>
  </select>

  <br><br>
  Marital Status:
  <input type="radio" value="maried" id="maried" v-model="person.maritalStatus">
  <label for="maried">Married</label>
  <input type="radio" value="single" id="single" v-model="person.maritalStatus">
  <label for="single">Single</label>
  <input type="radio" value="divorced" id="divorced" v-model="person.maritalStatus">
  <label for="divorced">Divorced</label>
  <input type="radio" value="widower" id="widower" v-model="person.maritalStatus">
  <label for="widower">Widowed</label>

  <br><br>
  <h3>Reactive Variables</h3>
  name: <b>{{person.name}}</b>
  <br><br>
  birthdate: <b>{{person.birthdate}}</b>
  <br><br>
  maritalStatus: <b>{{person.maritalStatus}}</b>

</template>

<style scoped>
  h3 {
    background-color: gainsboro;
    padding: 5px;
  }
</style>
