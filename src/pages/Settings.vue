```vue
<template>
  <div class="settings-page">
    <!-- Header -->
    <header-component
        :showBackButton="true"
        @logout="handleLogout"
    />

    <!-- Settings Content -->
    <div class="settings-content">
      <div class="settings-container">

        <!-- Page Header -->
        <div class="page-header">
          <div>
            <h1>Settings</h1>
            <p>Manage your rent-a-car POS system and preferences</p>
          </div>
        </div>

        <!-- Settings Grid -->
        <div class="settings-grid">

          <!-- Business Information -->
          <router-link to="/settings/business-information" class="settings-card">
            <div class="settings-icon business">
              <i class="fa fa-building"></i>
            </div>

            <div class="settings-card-content">
              <h3>Business Information</h3>
              <p>
                Manage company name, address, contact details and business
                information.
              </p>
            </div>

            <i class="fa fa-chevron-right arrow"></i>
          </router-link>

          <!-- System Users -->
          <router-link to="/settings/system-users" class="settings-card">
            <div class="settings-icon users">
              <i class="fa fa-users"></i>
            </div>

            <div class="settings-card-content">
              <h3>System Users</h3>
              <p>
                Manage POS users, roles, permissions and account access.
              </p>
            </div>

            <i class="fa fa-chevron-right arrow"></i>
          </router-link>

          <!-- System Information -->
          <router-link to="/settings/system-information" class="settings-card">
            <div class="settings-icon system">
              <i class="fa fa-info-circle"></i>
            </div>

            <div class="settings-card-content">
              <h3>System Information</h3>
              <p>
                View application version, developer information, license and
                support details.
              </p>
            </div>

            <i class="fa fa-chevron-right arrow"></i>
          </router-link>

        </div>

      </div>
    </div>
  </div>
</template>

<script>
import HeaderComponent from '../component/Header.vue';

export default {
  name: 'Settings',

  components: {
    HeaderComponent
  },

  methods: {
    handleLogout() {
      this.$emit('logout');
    }
  }
};
</script>

<style scoped>
.settings-page {
  min-height: 100vh;
  background: var(--color-background-tertiary);
}

.settings-content {
  padding: 2rem 0;
}

.settings-container {
  max-width: 1200px;
  margin: 0 auto;
  padding: 0 2rem;
}

/* Page Header */

.page-header {
  margin-bottom: 2rem;
}

.page-header h1 {
  margin: 0 0 0.5rem 0;
  font-size: 28px;
  font-weight: 600;
  color: var(--color-text-primary);
}

.page-header p {
  margin: 0;
  font-size: 14px;
  color: var(--color-text-secondary);
}

/* Settings Grid */

.settings-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 1.25rem;
}

/* Settings Card */

.settings-card {
  position: relative;
  display: flex;
  align-items: center;
  gap: 1.25rem;

  min-height: 130px;
  padding: 1.5rem;

  background: var(--color-background-primary);
  border: 1px solid var(--color-border-tertiary);
  border-radius: var(--border-radius-lg);

  text-decoration: none;
  color: var(--color-text-primary);

  transition: all 0.2s ease;
}

.settings-card:hover {
  border-color: var(--color-border-secondary);
  background: var(--color-background-secondary);
  transform: translateY(-2px);

  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.06);
}

/* Icon */

.settings-icon {
  width: 58px;
  height: 58px;

  flex-shrink: 0;

  display: flex;
  align-items: center;
  justify-content: center;

  border-radius: var(--border-radius-md);

  color: white;
  font-size: 26px;
}

.settings-icon.business {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
}

.settings-icon.rental {
  background: linear-gradient(135deg, #4facfe 0%, #00f2fe 100%);
}

.settings-icon.invoice {
  background: linear-gradient(135deg, #43e97b 0%, #38f9d7 100%);
}

.settings-icon.users {
  background: linear-gradient(135deg, #fa709a 0%, #fee140 100%);
}

.settings-icon.security {
  background: linear-gradient(135deg, #30cfd0 0%, #330867 100%);
}

.settings-icon.backup {
  background: linear-gradient(135deg, #a8edea 0%, #fed6e3 100%);
}

.settings-icon.system {
  background: linear-gradient(135deg, #667eea 0%, #4facfe 100%);
}

/* Card Content */

.settings-card-content {
  flex: 1;
  min-width: 0;
}

.settings-card-content h3 {
  margin: 0 0 0.5rem 0;

  font-size: 16px;
  font-weight: 600;

  color: var(--color-text-primary);
}

.settings-card-content p {
  margin: 0;

  font-size: 13px;
  line-height: 1.5;

  color: var(--color-text-secondary);
}

/* Arrow */

.arrow {
  flex-shrink: 0;

  font-size: 14px;

  color: var(--color-text-secondary);

  transition: transform 0.2s ease;
}

.settings-card:hover .arrow {
  transform: translateX(4px);
}

/* Mobile */

@media (max-width: 768px) {
  .settings-container {
    padding: 0 1rem;
  }

  .settings-content {
    padding: 1.5rem 0;
  }

  .settings-grid {
    grid-template-columns: 1fr;
  }

  .page-header h1 {
    font-size: 24px;
  }

  .settings-card {
    min-height: 110px;
    padding: 1.25rem;
  }

  .settings-icon {
    width: 50px;
    height: 50px;
    font-size: 22px;
  }
}
</style>
```
