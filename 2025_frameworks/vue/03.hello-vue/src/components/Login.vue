<script setup lang="ts">
</script>

<template>
  <!--div
    class="flex items-center justify-center gap-8 min-h-screen bg-gradient-to-br from-green-500 to-sky-400"
  >
    <p> Hello, world!</p>
  </div-->

  <div class="login-page bg-gradient-to-br from-green-500 to-sky-400">
    <div class="login-container">
      <span> Account </span>
      <input
        type="text" id="account" name="account" required minlength="4" maxlength="32" size="10"
        placeholder="phone or email"
      />
      <span> Password </span>
      <input
        type="password" id="password" name="password" required minlength="8" maxlength="32" size="10"
        placeholder="password"
      />
      <button class="login-button"> Login </button>
    </div>
  </div>
</template>


<style>
.login-page {
  height: 100vh;
  display: flex;
  justify-content: flex-end;
  align-items: center;
  background-color: white;
}

.login-container {
  border: 1px solid grey;
  border-radius: 5px;
  padding: 2rem 2rem;
  margin: 1rem 1rem;
  display: grid;
  row-gap: 1rem;
  column-gap: 0.5rem;
  grid-template-columns: 1fr 3fr;
  /*justify-content: center;*/
  align-items: center;
  font-size: 1.5rem;
}

.login-container > input {
  border: 1px solid blue;
  padding: 0.5rem;
  min-width: 15rem;
}

.login-button {
  background-color: green;
  border-radius: 5px;
  padding: 0.2rem;
  font-size: 1.8rem;
  color: white;
  grid-column: 1 / span 2;
}

.login-container > span {
  text-align: right;
}
</style>
