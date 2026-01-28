<script setup lang="ts">
type Props = {
  disabled?: boolean
  testId?: string
}

defineProps<Props>()

defineEmits<{ (e: "click", event: MouseEvent): void }>()
</script>

<template>
  <button
    class="btn btn-coral"
    type="button"
    :disabled="disabled"
    :data-testid="testId"
    @click="$emit('click', $event)"
  >
    <slot />
  </button>
</template>
