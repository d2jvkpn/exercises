<script setup>
  import { defineProps, ref, onMounted, watch, watchEffect, inject } from "vue";

  const props = defineProps(["name"]);

  //
  const names = ref([]);
  let countries= [];

  //
  const handleData = data => {
    countries = JSON.parse(data).map(function(elem) {
      let name = elem.name.common;

      if (elem.cca2) {
        name += `(${elem.cca2})`;
      }

      return name;
    });

    // In ascending alphabetical order
    countries = countries.sort((n1, n2) => {
      if (n1 > n2) { return 1 }
      if (n1 < n2) { return -1 }

      return 0;
    });

    names.value = countries; // Updating the displayed list.
  };

  onMounted(() => {
    var url = "https://restcountries.com/v3.1/all";

    fetch(url).then(res => res.text())
      .then(handleData)
      .catch(err => {
        /*names.value = [err.toString()]; */
        let msg = "";

        if (err instanceof TypeError && err.message.startsWith("NetworkError")) {
          msg = `NetworkError: request failed`;
        } else if (err instanceof TypeError) {
          msg = `TypeError: ${err.message}`;
        } else if (err instanceof SyntaxError) {
          msg = `SyntaxError: invalid response data`;
        } else {
          msg = `UnexpectedError: ${err.message}`;
        }

        alert(msg);
      });
  });

  async function getCountries() {
    var url = "https://restcountries.com/v3.1/all";
    var response = await fetch(url);
    var data = await response.text();

    countries = JSON.parse(data).map(elem => {
      let name = elem.name.common;

      if (elem.cca2) {
        name += `(${elem.cca2})`;
      }

      return name;
    });

    countries = countries.sort((n1, n2) => {
      if (n1 > n2) return 1;
      if (n1 < n2) return -1;

      return 0;
    });

    return countries;
  }

  onMounted(async () => names.value = await getCountries());

  const countryRegExp = newName => {
    if (/^[A-Z]{2}$/.test(newName)) {
      return new RegExp(`\\(${newName}\\)$`);
    } else {
      return new RegExp("^" + newName, "i");
    }
  }

  // 1. watch
  watch(() => props.name, newName => {
    // console.log(`~~~ 1. watch: ${props.name}`);

    if (!newName) {
      names.value = countries;
      return;
    }

    // const reg = new RegExp("^" + newName, "i");
    const reg = countryRegExp(newName);

    names.value = countries.filter(val => val.match(reg) ? true : false);
  });

  // 2. watchEffect
  watchEffect(() => {
    // Do not delete: allows the observation of props.name
    // console.log(`~~~ 2. watchEffect: ${props.name}`);

    // const reg = new RegExp("^" + props.name, "i");
    const reg = countryRegExp(props.name);

    names.value = countries.filter(val => val.match(reg) ? true : false);
  });

  // 3. watch inject
  const name = inject("name"); // The reactive variable name is retrieved

  watch(name, () => {
    // console.log(`~~~ 3. watch inject: ${name.value}`);

    // const reg = new RegExp("^" + name.value, "i");
    const reg = countryRegExp(props.name);

    names.value = countries.filter(val => val.match(reg) ? true : false);
  });
</script>

<template>
  <h3>Country List</h3>

  <div v-show="countries.length == 0">Fetching countries in progress...</div>

  Entered Country: {{name}}
  <ul>
    <li v-for="name in names" :key="name" >{{name}}</li>
  </ul>
</template>
