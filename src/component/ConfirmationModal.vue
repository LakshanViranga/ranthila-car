<template>
  <v-dialog
      v-model="isOpen"
      max-width="420px"
      persistent
      transition="fade-transition"
      @update:model-value="handleDialogClose"
  >
    <v-card class="confirmation-modal">
      <!-- Icon & Header -->
      <div class="confirmation-header">
        <div class="confirmation-icon" :style="{ background: iconBg, color: iconColor }">
          <v-icon :icon="icon" size="24" />
        </div>

        <div class="confirmation-text">
          <h2 class="confirmation-title">{{ title }}</h2>
          <p v-if="subtitle" class="confirmation-subtitle">{{ subtitle }}</p>
        </div>
      </div>

      <!-- Message -->
      <v-card-text class="confirmation-message">
        {{ message }}
      </v-card-text>

      <!-- Actions -->
      <v-card-actions class="confirmation-actions">
        <v-spacer />
        <v-btn
            variant="text"
            :disabled="isLoading"
            @click="handleCancel"
        >
          {{ cancelText }}
        </v-btn>

        <v-btn
            :color="buttonColor"
            variant="flat"
            :loading="isLoading"
            @click="handleConfirm"
        >
          {{ confirmText }}
        </v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script setup lang="ts">
import { ref } from 'vue'

interface DialogConfig {
  title: string
  message: string
  subtitle?: string
  confirmText?: string
  cancelText?: string
  icon?: string
  type?: 'default' | 'success' | 'warning' | 'error' | 'info'
  onConfirm?: () => Promise<void> | void
  onCancel?: () => void
}

const isOpen = ref(false)
const isLoading = ref(false)
const config = ref<DialogConfig>({
  title: 'Confirm',
  message: 'Are you sure?',
})

let onConfirmCallback: (() => Promise<void> | void) | null = null
let onCancelCallback: (() => void) | null = null

const typeStyles = {
  default: { bg: '#f3f4f6', color: '#dee469', btn: 'primary' },
  success: { bg: '#dcfce7', color: '#16a34a', btn: 'success' },
  warning: { bg: '#fef3c7', color: '#f59e0b', btn: 'warning' },
  error: { bg: '#fee2e2', color: '#dc2626', btn: 'error' },
  info: { bg: '#dbeafe', color: '#0284c7', btn: 'info' },
}

const type = ref<keyof typeof typeStyles>('default')
const iconBg = ref('#f3f4f6')
const iconColor = ref('#6b7280')
const buttonColor = ref('primary')

const title = ref('')
const subtitle = ref('')
const message = ref('')
const confirmText = ref('Confirm')
const cancelText = ref('Cancel')
const icon = ref('fa fa-help')

const open = (cfg: DialogConfig) => {
  config.value = cfg
  title.value = cfg.title
  message.value = cfg.message
  subtitle.value = cfg.subtitle || ''
  confirmText.value = cfg.confirmText || 'Confirm'
  cancelText.value = cfg.cancelText || 'Cancel'

  type.value = cfg.type || 'default'
  const style = typeStyles[type.value]

  iconBg.value = style.bg
  iconColor.value = style.color
  buttonColor.value = style.btn

  icon.value = cfg.icon || getDefaultIcon(type.value)
  onConfirmCallback = cfg.onConfirm || null
  onCancelCallback = cfg.onCancel || null

  isOpen.value = true
}

const close = () => {
  isOpen.value = false
  isLoading.value = false
}

const handleConfirm = async () => {
  if (!onConfirmCallback) {
    close()
    return
  }

  isLoading.value = true
  try {
    await Promise.resolve(onConfirmCallback())
    close()
  } catch (error) {
    console.error('Confirmation error:', error)
    isLoading.value = false
  }
}

const handleCancel = () => {
  onCancelCallback?.()
  close()
}

const handleDialogClose = (value: boolean) => {
  if (!value) {
    handleCancel()
  }
}

const getDefaultIcon = (typeVal: keyof typeof typeStyles) => {
  const icons = {
    default: 'fa fa-question-circle',
    success: 'fa fa-check-circle',
    warning: 'fa fa-exclamation-circle',
    error: 'fa fa-exclamation-circle',
    info: 'fa fa-info-circle',
  }
  return icons[typeVal]
}

defineExpose({ open, close })
</script>

<style scoped>
.confirmation-modal {
  border-radius: 12px !important;
  overflow: hidden;
  border: none;
  box-shadow: 0 10px 30px rgba(0, 0, 0, 0.12) !important;
}

.confirmation-header {
  display: flex;
  align-items: flex-start;
  gap: 16px;
  padding: 24px;
  background: #ffffff;
}

.confirmation-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 48px;
  height: 48px;
  min-width: 48px;
  border-radius: 10px;
  transition: transform 0.2s ease;
}

.confirmation-header:hover .confirmation-icon {
  transform: scale(1.05);
}

.confirmation-text {
  flex: 1;
  min-width: 0;
}

.confirmation-title {
  margin: 0;
  font-size: 18px;
  font-weight: 600;
  color: #1f2937;
  letter-spacing: -0.3px;
}

.confirmation-subtitle {
  margin: 4px 0 0;
  font-size: 13px;
  color: #9ca3af;
  font-weight: 500;
}

.confirmation-message {
  padding: 0 24px 20px !important;
  font-size: 14px;
  line-height: 1.6;
  color: #6b7280;
  letter-spacing: -0.2px;
  background: #ffffff;
}

.confirmation-actions {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  padding: 16px 24px 20px !important;
  border-top: 1px solid #f0f0f0;
  background: #f9fafb;
}

.confirmation-actions :deep(.v-btn) {
  border-radius: 8px;
  text-transform: none;
  font-weight: 500;
  min-width: 80px;
  transition: all 0.2s ease;
}

.confirmation-actions :deep(.v-btn--text) {
  color: #6b7280;
}

.confirmation-actions :deep(.v-btn--text:hover) {
  background-color: #f3f4f6;
}

.confirmation-actions :deep(.v-btn--flat) {
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
}

.confirmation-actions :deep(.v-btn--flat:hover) {
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.12);
  transform: translateY(-1px);
}

/* Dialog transition animation */
:deep(.fade-transition-enter-active),
:deep(.fade-transition-leave-active) {
  transition: opacity 0.2s ease;
}

:deep(.fade-transition-enter-from),
:deep(.fade-transition-leave-to) {
  opacity: 0;
}
</style>
