<template>
  <v-snackbar
      v-model="snackbar.show"
      :color="snackbar.type"
      :timeout="6000"
      location="bottom"
  >
    {{ snackbar.message }}

    <template #actions>
      <v-btn
          variant="text"
          @click="snackbar.show = false"
      >
        Close
      </v-btn>
    </template>
  </v-snackbar>
</template>

<script setup>
import { useSnackbar } from '../composables/useSnackbar';

const { snackbar } = useSnackbar();
</script>
