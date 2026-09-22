<template>
  <div class="system-information-page">
    <!-- Header -->
    <header-component
        :showBackButton="true"
        @logout="handleLogout"
    />

    <!-- Content -->
    <div class="system-information-content">
      <div class="system-information-container">

        <!-- Page Header -->
        <div class="page-header">
          <div>
            <h1>System Information</h1>
            <p>Application, developer and system details</p>
          </div>
        </div>

        <!-- Application Information -->
        <section class="information-section">
          <div class="section-header">
            <div class="section-icon application">
              <i class="fa fa-desktop"></i>
            </div>

            <div>
              <h2>Application Information</h2>
              <p>Information about your rent-a-car POS application</p>
            </div>
          </div>

          <div class="information-grid">

            <div class="information-item">
              <span class="item-label">Application Name</span>
              <span class="item-value">RentCar POS</span>
            </div>

            <div class="information-item">
              <span class="item-label">Version</span>
              <span class="item-value">1.0.0</span>
            </div>

            <div class="information-item">
              <span class="item-label">Build</span>
              <span class="item-value">2026.09.01</span>
            </div>

            <div class="information-item">
              <span class="item-label">Environment</span>
              <span class="item-value">
                <span class="status-badge">
                  Production
                </span>
              </span>
            </div>

          </div>
        </section>

        <!-- Developer Information -->
        <section class="information-section">
          <div class="section-header">
            <div class="section-icon developer">
              <i class="fa fa-code"></i>
            </div>

            <div>
              <h2>Developer Information</h2>
              <p>Software developer and support information</p>
            </div>
          </div>

          <div class="information-grid">

            <div class="information-item">
              <span class="item-label">Developed By</span>
              <span class="item-value">Your Company Name</span>
            </div>

            <div class="information-item">
              <span class="item-label">Developer</span>
              <span class="item-value">Your Developer Name</span>
            </div>

            <div class="information-item">
              <span class="item-label">Website</span>
              <span class="item-value link">
                https://yourwebsite.com
              </span>
            </div>

            <div class="information-item">
              <span class="item-label">Support Email</span>
              <span class="item-value link">
                support@yourcompany.com
              </span>
            </div>

            <div class="information-item">
              <span class="item-label">Support Phone</span>
              <span class="item-value">
                +94 XX XXX XXXX
              </span>
            </div>

          </div>
        </section>

        <!-- License Information -->
        <section class="information-section">
          <div class="section-header">
            <div class="section-icon license">
              <i class="fa fa-key"></i>
            </div>

            <div>
              <h2>License Information</h2>
              <p>Application license and activation details</p>
            </div>
          </div>

          <div class="information-grid">

            <div class="information-item">
              <span class="item-label">License Type</span>
              <span class="item-value">Commercial</span>
            </div>

            <div class="information-item">
              <span class="item-label">License Status</span>
              <span class="item-value">
                <span class="status-badge active">
                  <i class="fa fa-check-circle"></i>
                  Active
                </span>
              </span>
            </div>

            <div class="information-item">
              <span class="item-label">Licensed To</span>
              <span class="item-value">
                Your Rental Company
              </span>
            </div>

            <div class="information-item">
              <span class="item-label">License Expiry</span>
              <span class="item-value">
                Perpetual
              </span>
            </div>

          </div>
        </section>

        <!-- System Details -->
        <section class="information-section">
          <div class="section-header">
            <div class="section-icon system">
              <i class="fa fa-server"></i>
            </div>

            <div>
              <h2>System Details</h2>
              <p>Technical information about the current installation</p>
            </div>
          </div>

          <div class="information-grid">

            <div class="information-item">
              <span class="item-label">Database</span>
              <span class="item-value">Local Database</span>
            </div>

            <div class="information-item">
              <span class="item-label">Database Version</span>
              <span class="item-value">1.0</span>
            </div>

            <div class="information-item">
              <span class="item-label">Platform</span>
              <span class="item-value">Desktop POS</span>
            </div>

            <div class="information-item">
              <span class="item-label">Last Updated</span>
              <span class="item-value">September 1, 2026</span>
            </div>

          </div>
        </section>

        <!-- About -->
        <section class="about-section">

          <div class="about-logo">
            <i class="fa fa-car"></i>
          </div>

          <h2>RentCar POS</h2>

          <p>
            Vehicle Rental Management System
          </p>

          <span class="copyright">
            © 2026 Your Company Name. All rights reserved.
          </span>

        </section>

      </div>
    </div>
  </div>
</template>

<script>
import HeaderComponent from '../component/Header.vue';

export default {
  name: 'SystemInformation',

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
.system-information-page {
  min-height: 100vh;
  background: var(--color-background-tertiary);
}

.system-information-content {
  padding: 2rem 0;
}

.system-information-container {
  max-width: 1000px;
  margin: 0 auto;
  padding: 0 2rem;

  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

/* Page Header */

.page-header {
  margin-bottom: 0.5rem;
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

/* Information Section */

.information-section {
  background: var(--color-background-primary);

  border: 1px solid var(--color-border-tertiary);
  border-radius: var(--border-radius-lg);

  overflow: hidden;
}

/* Section Header */

.section-header {
  display: flex;
  align-items: center;

  gap: 1rem;

  padding: 1.25rem 1.5rem;

  border-bottom: 1px solid var(--color-border-tertiary);
}

.section-header h2 {
  margin: 0 0 0.25rem 0;

  font-size: 17px;
  font-weight: 600;

  color: var(--color-text-primary);
}

.section-header p {
  margin: 0;

  font-size: 13px;

  color: var(--color-text-secondary);
}

/* Section Icons */

.section-icon {
  width: 46px;
  height: 46px;

  flex-shrink: 0;

  display: flex;
  align-items: center;
  justify-content: center;

  border-radius: var(--border-radius-md);

  color: white;
  font-size: 20px;
}

.section-icon.application {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
}

.section-icon.developer {
  background: linear-gradient(135deg, #4facfe 0%, #00f2fe 100%);
}

.section-icon.license {
  background: linear-gradient(135deg, #43e97b 0%, #38f9d7 100%);
}

.section-icon.system {
  background: linear-gradient(135deg, #fa709a 0%, #fee140 100%);
}

/* Information Grid */

.information-grid {
  display: grid;

  grid-template-columns: repeat(2, 1fr);

  gap: 0;

  padding: 0 1.5rem;
}

/* Information Item */

.information-item {
  display: flex;
  flex-direction: column;

  gap: 0.4rem;

  padding: 1.15rem 1rem;

  border-bottom: 1px solid var(--color-border-tertiary);
}

.information-item:nth-last-child(-n + 2) {
  border-bottom: none;
}

.item-label {
  font-size: 12px;

  color: var(--color-text-secondary);
}

.item-value {
  font-size: 14px;
  font-weight: 500;

  color: var(--color-text-primary);

  word-break: break-word;
}

.item-value.link {
  color: #667eea;
}

/* Status */

.status-badge {
  display: inline-flex;

  align-items: center;

  width: fit-content;

  padding: 0.35rem 0.65rem;

  border-radius: 999px;

  background: var(--color-background-secondary);

  font-size: 12px;
  font-weight: 500;
}

.status-badge.active {
  color: #218838;
}

.status-badge i {
  margin-right: 0.35rem;
}

/* About Section */

.about-section {
  display: flex;
  flex-direction: column;

  align-items: center;
  text-align: center;

  padding: 2.5rem 1.5rem;

  background: var(--color-background-primary);

  border: 1px solid var(--color-border-tertiary);

  border-radius: var(--border-radius-lg);
}

.about-logo {
  width: 64px;
  height: 64px;

  display: flex;
  align-items: center;
  justify-content: center;

  margin-bottom: 1rem;

  border-radius: var(--border-radius-lg);

  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);

  color: white;

  font-size: 28px;
}

.about-section h2 {
  margin: 0 0 0.35rem 0;

  font-size: 20px;
  font-weight: 600;

  color: var(--color-text-primary);
}

.about-section p {
  margin: 0 0 0.75rem 0;

  font-size: 13px;

  color: var(--color-text-secondary);
}

.copyright {
  font-size: 12px;

  color: var(--color-text-secondary);
}

/* Mobile */

@media (max-width: 768px) {

  .system-information-content {
    padding: 1.5rem 0;
  }

  .system-information-container {
    padding: 0 1rem;
  }

  .page-header h1 {
    font-size: 24px;
  }

  .information-grid {
    grid-template-columns: 1fr;

    padding: 0 1rem;
  }

  .information-item:nth-last-child(-n + 2) {
    border-bottom: 1px solid var(--color-border-tertiary);
  }

  .information-item:last-child {
    border-bottom: none;
  }

  .section-header {
    padding: 1rem;
  }

  .section-icon {
    width: 42px;
    height: 42px;
  }
}
</style>


