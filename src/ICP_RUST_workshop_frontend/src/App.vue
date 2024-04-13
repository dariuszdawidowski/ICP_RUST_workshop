<script lang="ts">
import { ref } from 'vue';
import { ICP_RUST_workshop_backend } from 'declarations/ICP_RUST_workshop_backend/index';
let greeting = ref('');

export default {
  data() {

    return {
      greeting: '',
      wpisy: [],
      nowyWpis: ''
    }

  },

  methods: {

    async handleSubmit(e) {
      e.preventDefault();
      const target = e.target;
      const name = target.querySelector('#name').value;
      await ICP_RUST_workshop_backend.greet(name).then((response) => {
        greeting.value = response;
      });
    },

    async pobierzWpisy() {
      this.wpisy = await ICP_RUST_workshop_backend.pobierz_wpisy();
    },

    async dodajWpis() {
      if (this.nowyWpis.trim() === '') return;
      await ICP_RUST_workshop_backend.dodaj_wpis(this.nowyWpis);
      await this.pobierzWpisy();
    },

  },

  mounted() {
    this.pobierzWpisy();
  }
}
</script>

<template>
  <main>
    <img src="/logo2.svg" alt="DFINITY logo" />
    <br />
    <br />
    <form action="#" @submit="handleSubmit">
      <label for="name">Enter your name: &nbsp;</label>
      <input id="name" alt="Name" type="text" />
      <button type="submit">Click Me!</button>
    </form>
    <section id="greeting">{{ greeting }}</section>
    <div>
      <input v-model="nowyWpis">
      <button @click="dodajWpis()">Zapisz</button>
    </div>
    <div>{{ wpisy }}</div>
  </main>
</template>
