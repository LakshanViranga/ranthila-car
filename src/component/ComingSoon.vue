<template>
  <div class="coming-soon-container" :class="containerClass">
    <!-- Main Card -->
    <div class="coming-soon-card">
      <!-- Flag Icon -->
      <div class="flag-wrapper">
        <span class="flag-emoji" :title="flagTitle">{{ flag }}</span>
      </div>

      <!-- Content -->
      <div class="content">
        <!-- Title (Optional) -->
        <h3 v-if="title" class="title">{{ title }}</h3>

        <!-- Main Message -->
        <p class="message">{{ message }}</p>

        <!-- Subtitle (Optional) -->
        <p v-if="subtitle" class="subtitle">{{ subtitle }}</p>

        <!-- CTA Button (Optional) -->
        <button
            v-if="showNotifyButton"
            class="notify-button"
            @click="handleNotify"
        >
          📧 {{ notifyButtonText }}
        </button>
      </div>

      <!-- Sparkle Animation (Visual Enhancement) -->
      <div class="sparkles">
        <span class="sparkle">✨</span>
        <span class="sparkle">✨</span>
        <span class="sparkle">✨</span>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed } from 'vue'

// Props
const props = defineProps({
  // Flag emoji or icon
  flag: {
    type: String,
    default: '🚀'
  },
  flagTitle: {
    type: String,
    default: 'Coming Soon'
  },
  // Main message
  message: {
    type: String,
    default: 'This will be available in the next update'
  },
  // Optional title
  title: {
    type: String,
    default: ''
  },
  // Optional subtitle
  subtitle: {
    type: String,
    default: ''
  },
  // Show notify button
  showNotifyButton: {
    type: Boolean,
    default: false
  },
  notifyButtonText: {
    type: String,
    default: 'Notify Me'
  },
  // Container variant
  variant: {
    type: String,
    enum: ['default', 'warning', 'success', 'info'],
    default: 'default'
  },
  // Size variant
  size: {
    type: String,
    enum: ['small', 'medium', 'large'],
    default: 'medium'
  },
  // Full height container
  fullHeight: {
    type: Boolean,
    default: false
  }
})

// Emits
const emit = defineEmits(['notify'])

// Methods
const handleNotify = () => {
  emit('notify')
}

// Computed
const containerClass = computed(() => {
  return {
    [`variant-${props.variant}`]: true,
    [`size-${props.size}`]: true,
    'full-height': props.fullHeight
  }
})
</script>

<style scoped>
/* Container */
.coming-soon-container {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 2rem;
  min-height: 200px;

  &.full-height {
    min-height: 100vh;
  }

  &.size-small {
    padding: 1rem;
    min-height: 150px;
  }

  &.size-large {
    padding: 3rem;
    min-height: 400px;
  }
}

/* Card Styling */
.coming-soon-card {
  position: relative;
  background: linear-gradient(135deg, #ffffff 0%, #f8f9fa 100%);
  border: 2px solid #e9ecef;
  border-radius: 16px;
  padding: 3rem 2rem;
  text-align: center;
  max-width: 500px;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.07);
  transition: all 0.3s ease;
  overflow: hidden;

  &:hover {
    box-shadow: 0 12px 24px rgba(0, 0, 0, 0.12);
    transform: translateY(-2px);
  }
}

/* Flag Wrapper */
.flag-wrapper {
  margin-bottom: 1.5rem;
  display: inline-block;
}

.flag-emoji {
  font-size: 4rem;
  display: block;
  animation: float 3s ease-in-out infinite;

  @keyframes float {
    0%, 100% {
      transform: translateY(0px);
    }
    50% {
      transform: translateY(-10px);
    }
  }
}

/* Content */
.content {
  position: relative;
  z-index: 2;
}

.title {
  font-size: 1.5rem;
  font-weight: 700;
  color: #2d3748;
  margin: 0 0 0.5rem 0;
  letter-spacing: -0.5px;
}

.message {
  font-size: 1.1rem;
  color: #4a5568;
  margin: 1rem 0;
  line-height: 1.6;
  font-weight: 500;
}

.subtitle {
  font-size: 0.95rem;
  color: #718096;
  margin: 0.75rem 0 0 0;
  line-height: 1.5;
  font-style: italic;
}

/* Notify Button */
.notify-button {
  margin-top: 1.5rem;
  padding: 0.75rem 1.5rem;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border: none;
  border-radius: 8px;
  font-size: 0.95rem;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.3s ease;
  box-shadow: 0 4px 12px rgba(102, 126, 234, 0.4);

  &:hover {
    transform: translateY(-2px);
    box-shadow: 0 6px 20px rgba(102, 126, 234, 0.6);
  }

  &:active {
    transform: translateY(0);
  }
}

/* Sparkles Animation */
.sparkles {
  position: absolute;
  width: 100%;
  height: 100%;
  top: 0;
  left: 0;
  pointer-events: none;
  overflow: hidden;
}

.sparkle {
  position: absolute;
  font-size: 1.5rem;
  animation: sparkle 2s ease-in-out infinite;
  opacity: 0;

  &:nth-child(1) {
    top: 10%;
    left: 10%;
    animation-delay: 0s;
  }

  &:nth-child(2) {
    top: 70%;
    right: 15%;
    animation-delay: 0.5s;
  }

  &:nth-child(3) {
    bottom: 20%;
    left: 20%;
    animation-delay: 1s;
  }

  @keyframes sparkle {
    0%, 100% {
      opacity: 0;
      transform: scale(0);
    }
    50% {
      opacity: 1;
      transform: scale(1);
    }
  }
}

/* Variants */
.variant-default {
  .coming-soon-card {
    border-color: #e9ecef;
    background: linear-gradient(135deg, #ffffff 0%, #f8f9fa 100%);
  }

  .message {
    color: #4a5568;
  }
}

.variant-warning {
  .coming-soon-card {
    border-color: #fcd34d;
    background: linear-gradient(135deg, #fffbeb 0%, #fef3c7 100%);
  }

  .message {
    color: #92400e;
  }

  .title {
    color: #b45309;
  }
}

.variant-success {
  .coming-soon-card {
    border-color: #86efac;
    background: linear-gradient(135deg, #f0fdf4 0%, #dcfce7 100%);
  }

  .message {
    color: #166534;
  }

  .title {
    color: #15803d;
  }
}

.variant-info {
  .coming-soon-card {
    border-color: #93c5fd;
    background: linear-gradient(135deg, #eff6ff 0%, #dbeafe 100%);
  }

  .message {
    color: #1e40af;
  }

  .title {
    color: #1e3a8a;
  }
}

/* Responsive */
@media (max-width: 640px) {
  .coming-soon-container {
    padding: 1rem;
  }

  .coming-soon-card {
    padding: 2rem 1.5rem;
  }

  .flag-emoji {
    font-size: 3rem;
  }

  .title {
    font-size: 1.25rem;
  }

  .message {
    font-size: 1rem;
  }
}
</style>
