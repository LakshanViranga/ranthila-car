<template>
  <div class="account-page">
    <header-component
        :userName="`Logged in as ${loggedUser}`"
        :showBackButton="true"
        @logout="handleLogout"
    />

    <div class="account-content">
      <div class="account-container">

        <!-- Page Header -->
        <div class="page-header">
          <div class="header-title">
            <div class="date-navigation">
              <button
                  class="btn-date-nav btn-prev"
                  @click="previousDate"
                  title="Previous day"
                  :disabled="!canGoToPreviousDate"
              >
                <i class="fa fa-chevron-left"></i>
              </button>

              <h1>{{ getDisplayHeadingText() }}</h1>

              <button
                  class="btn-date-nav btn-next"
                  @click="nextDate"
                  title="Next day"
                  :disabled="!canGoToNextDate"
              >
                <i class="fa fa-chevron-right"></i>
              </button>
            </div>
            <p class="page-subtitle">
              {{ t('accountPage.subHeading') }}
            </p>
          </div>

          <div class="header-actions">
            <button
                class="btn-action-danger"
                @click="openDaySettleModal()"
                v-if="disabledDaySettle"
            >
              <i class="fa fa-minus"></i>
              {{ haveDifference.totalIncomeDifference || haveDifference.totalExpenseDifference ?  t('accountPage.button.dayResettle') : t('accountPage.button.daySettle')}}
            </button>

            <button
                class="btn-action-bank"
                @click="openAddRecordModal('deposit')"
            >
              <i class="fa fa-university"></i>
              {{ t('accountPage.button.bankDeposit') }}
            </button>
          </div>
        </div>

        <!-- Account Summary -->
        <div class="stats-section">
          <div class="stat-card cash-balance-card" :class="{ 'card-mismatch' : haveDifference.totalIncomeDifference}">
            <div class="stat-icon">
              <i class="fa fa-money"></i>
            </div>
            <div>
              <label>{{ getDisplayDateText() }} {{ t('accountPage.totalIncome')}}</label>
              <span class="stat-value">{{ formatPrice(totalIncome) }}</span>
              <span v-if="haveDifference.totalIncomeDifference" class="stat-previous">{{ t('accountPage.previousValue', { value : formatPrice(savedSummary.savedTotalIncome)} ) }}</span>
            </div>
            <div v-if="haveDifference.totalIncomeDifference" class="mismatch-badge">!</div>
          </div>

          <div class="stat-card credit-balance-card" :class="{ 'card-mismatch' :haveDifference.totalExpenseDifference }">
            <div class="stat-icon">
              <i class="fa fa-credit-card"></i>
            </div>
            <div>
              <label>{{ getDisplayDateText() }} Total Expenses</label>
              <span class="stat-value">{{ formatPrice(totalExpenses) }}</span>
              <span v-if="haveDifference.totalExpenseDifference" class="stat-previous">Previous Value :{{ formatPrice(savedSummary.savedExpense) }}</span>
            </div>
            <div v-if="haveDifference.totalExpenseDifference" class="mismatch-badge">!</div>
          </div>

          <div class="stat-card income-card" :class="{ 'card-mismatch' :haveDifference.totalIncomeCashDifference }">
            <div class="stat-icon">
              <i class="fa fa-arrow-down"></i>
            </div>
            <div>
              <label>{{ getDisplayDateText() }} Cash Income</label>
              <span class="stat-value">{{ formatPrice(totalCashIncome) }}</span>
              <span v-if="haveDifference.totalIncomeCashDifference" class="stat-previous">Previous Value :{{ formatPrice(savedSummary.savedIncomeCash) }}</span>
            </div>
            <div v-if="haveDifference.totalIncomeCashDifference" class="mismatch-badge">!</div>
          </div>

          <div class="stat-card income-card" :class="{ 'card-mismatch' :haveDifference.totalIncomeCreditDifference }">
            <div class="stat-icon">
              <i class="fa fa-credit-card"></i>
            </div>
            <div>
              <label>{{ getDisplayDateText() }} Credit Income</label>
              <span class="stat-value">{{ formatPrice(totalCreditIncome) }}</span>
              <span v-if="haveDifference.totalIncomeCreditDifference" class="stat-previous">Previous Value :{{ formatPrice(savedSummary.savedIncomeCredit) }}</span>
            </div>
            <div v-if="haveDifference.totalIncomeCreditDifference" class="mismatch-badge">!</div>
          </div>
          <div class="stat-card income-card" :class="{ 'card-mismatch' :haveDifference.totalIncomeBankDifference }">
            <div class="stat-icon">
              <i class="fa fa-credit-card"></i>
            </div>
            <div>
              <label>{{ getDisplayDateText() }} Bank Transfer Income</label>
              <span class="stat-value">{{ formatPrice(totalBankIncome) }}</span>
              <span v-if="haveDifference.totalIncomeBankDifference" class="stat-previous">Previous Value :{{ formatPrice(savedSummary.savedIncomeBank) }}</span>
            </div>
            <div v-if="haveDifference.totalIncomeBankDifference" class="mismatch-badge">!</div>
          </div>

          <div class="stat-card expense-card" :class="{ 'card-mismatch' :haveDifference.totalExpensesCashDifference }">
            <div class="stat-icon">
              <i class="fa fa-arrow-up"></i>
            </div>
            <div>
              <label>{{ getDisplayDateText() }} Cash Expenses</label>
              <span class="stat-value">{{ formatPrice(totalCashExpenses) }}</span>
              <span v-if="haveDifference.totalExpensesCashDifference" class="stat-previous">Previous Value :{{ formatPrice(savedSummary.savedExpensesCash) }}</span>
            </div>
            <div v-if="haveDifference.totalExpensesCashDifference" class="mismatch-badge">!</div>
          </div>

          <div class="stat-card expense-card" :class="{ 'card-mismatch' :haveDifference.totalExpensesCreditDifference }">
            <div class="stat-icon">
              <i class="fa fa-credit-card"></i>
            </div>
            <div>
              <label>{{ getDisplayDateText() }} Credit Expenses</label>
              <span class="stat-value">{{ formatPrice(totalCreditExpenses) }}</span>
              <span v-if="haveDifference.totalExpensesCreditDifference" class="stat-previous">Previous Value :{{ formatPrice(savedSummary.savedExpensesCredit) }}</span>
            </div>
            <div v-if="haveDifference.totalExpensesCreditDifference" class="mismatch-badge">!</div>
          </div>

          <div class="stat-card expense-card" :class="{ 'card-mismatch' :haveDifference.totalExpensesBankDifference }">
            <div class="stat-icon">
              <i class="fa fa-credit-card"></i>
            </div>
            <div>
              <label>{{ getDisplayDateText() }} Bank Transfer/Card Expenses</label>
              <span class="stat-value">{{ formatPrice(totalBankExpenses) }}</span>
              <span v-if="haveDifference.totalExpensesBankDifference" class="stat-previous">Previous Value :{{ formatPrice(savedSummary.savedExpensesBank) }}</span>
            </div>
            <div v-if="haveDifference.totalExpensesBankDifference" class="mismatch-badge">!</div>
          </div>

          <div class="stat-card deposit-card" :class="{ 'card-mismatch' :haveDifference.totalBankDepositDifference }">
            <div class="stat-icon">
              <i class="fa fa-university"></i>
            </div>
            <div>
              <label>{{ getDisplayDateText() }} Banked Amount</label>
              <span class="stat-value">{{ formatPrice(totalDeposits) }}</span>
              <span v-if="haveDifference.totalBankDepositDifference" class="stat-previous">Previous Value :{{ formatPrice(savedSummary.savedBankDeposit) }}</span>
            </div>
            <div v-if="haveDifference.totalBankDepositDifference" class="mismatch-badge">!</div>
          </div>

          <div class="stat-card net-card" :class="{ 'card-mismatch' :haveDifference.totalHandsOnCashDifference }">
            <div class="stat-icon">
              <i class="fa fa-calculator"></i>
            </div>
            <div>
              <label>{{ getDisplayDateText() }} Hand on Cash</label>
              <span class="stat-value">{{ formatPrice(handOnCash) }}</span>
              <span v-if="haveDifference.totalHandsOnCashDifference" class="stat-previous">Previous Value :{{ formatPrice(savedSummary.savedHandsOnCash) }}</span>
            </div>
            <div v-if="haveDifference.totalHandsOnCashDifference" class="mismatch-badge">!</div>
          </div>
        </div>

        <!-- Daily Account Records -->
        <div class="table-section">
          <div class="table-header">
            <div>
              <h2 class="section-title">Daily Account Records</h2>
              <p class="table-subtitle">
                Daily summary of income, expenses and bank deposits.
              </p>
            </div>

            <div class="table-filters">
              <input
                  v-model="searchQuery"
                  type="text"
                  class="search-input"
                  placeholder="Search records..."
              />

              <input
                  v-model="selectedDate"
                  type="date"
                  class="date-filter"
              />
            </div>
          </div>

          <div class="table-wrapper">
            <table class="account-table">
              <thead>
              <tr>
                <th rowspan="2">Date</th>
                <th colspan="3" class="group-expense">Expenses</th>
                <th colspan="3" class="group-income">Income</th>
                <th colspan="2" class="group-deposit">Daily Cash Settlement</th>
                <th rowspan="2">Actions</th>
              </tr>

              <tr>
                <th>Cash</th>
                <th>Credit</th>
                <th>Card/Bank Transfer</th>
                <th>Cash</th>
                <th>Credit</th>
                <th>Bank Transfer</th>
                <th>Banked</th>
                <th>In Hand</th>
              </tr>
              </thead>

              <tbody>
              <tr
                  v-for="record in filteredDailyRecords"
                  :key="record.id"
              >
                <td class="date-cell">
                  {{ formatDate(record.date) }}
                </td>

                <td class="amount-expense">
                  {{ formatPrice(record.expense_cash) }}
                </td>

                <td class="amount-credit-expense">
                  {{ formatPrice(record.expense_credit) }}
                </td>
                <td class="amount-credit-expense">
                  {{ formatPrice(record.expense_bank) }}
                </td>
                <td class="amount-income">
                  {{ formatPrice(record.income_cash) }}
                </td>

                <td class="amount-credit-income">
                  {{ formatPrice(record.income_credit) }}
                </td>

                <td class="amount-credit-income">
                  {{ formatPrice(record.income_bank) }}
                </td>

                <td class="amount-deposit">
                  {{ formatPrice(record.deposit_amount) }}
                </td>

                <td class="amount-cash">
                  {{ formatPrice(record.cash_amount) }}
                </td>

                <td class="actions">
                  <button
                      class="btn-action edit"
                      title="View details"
                      @click="viewRecord(record)"
                  >
                    <i class="fa fa-eye"></i>
                  </button>

                  <button
                      class="btn-action delete"
                      title="Delete"
                      @click="deleteRecord(record.id)"
                  >
                    <i class="fa fa-trash"></i>
                  </button>
                </td>
              </tr>
              </tbody>
            </table>

            <div
                v-if="filteredDailyRecords.length === 0"
                class="empty-state"
            >
              <i class="fa fa-calculator"></i>
              <p>No daily account records found</p>

              <button
                  v-if="records.length === 0"
                  class="btn-add-empty"
                  @click="openAddRecordModal('income')"
              >
                Add First Record
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Add Record Modal -->
    <div
        v-if="showRecordModal.value"
        class="modal-overlay"
        @click="closeRecordModal"
    >
      <div class="modal-content" @click.stop>
        <div class="modal-header">
          <div>
            <h2> Bank Deposit</h2>
            <p class="modal-subtitle">
              Add a bank deposits
            </p>
          </div>

          <button class="btn-close" @click="closeRecordModal">
            <i class="fa fa-times"></i>
          </button>
        </div>

        <div class="modal-body">
          <form @submit.prevent="saveRecord">
            <!-- Date / Amount -->
            <div class="form-row">
              <div class="form-group">
                <label>
                  Date <span class="required">*</span>
                </label>

                <input
                    v-model="recordForm.date"
                    type="date"
                    class="input-field"
                    required
                />
              </div>

              <div class="form-group">
                <label>
                  Amount (LKR) <span class="required">*</span>
                </label>

                <input
                    v-model.number="recordForm.amount"
                    type="number"
                    class="input-field"
                    min="0"
                    step="0.01"
                    placeholder="0.00"
                    required
                />
              </div>
            </div>

            <div class="deposit-info">
              <i class="fa fa-info-circle"></i>
              <span>
                  Record money transferred from daily cash into a bank
                  account.
                </span>
            </div>

            <div class="form-group">
              <label>
                Bank Account Name <span class="required">*</span>
              </label>

              <select
                  v-model="recordForm.bank_account"
                  class="input-field"
                  required
              >
                <option value="">Select bank account</option>
                <option value="Commercial Bank - Business">
                  Commercial Bank - Business
                </option>
                <option value="Sampath Bank - Business">
                  Sampath Bank - Business
                </option>
                <option value="BOC - Business">
                  BOC - Business
                </option>
                <option value="Other">Other</option>
              </select>
            </div>

            <div class="deposit-preview">
              <div>
                <span>Current Cash</span>
                <strong>{{ formatPrice(handOnCash.value) }}</strong>
              </div>

              <div class="arrow">
                <i class="fa fa-arrow-right"></i>
              </div>

              <div>
                <span>Cash After Deposit</span>
                <strong>
                  {{
                    formatPrice(
                        Math.max(
                            0,
                            handOnCash.value - Number(recordForm.amount || 0)
                        )
                    )
                  }}
                </strong>
              </div>
            </div>

            <div class="form-group">
              <label>Notes</label>
              <textarea
                  v-model="recordForm.notes"
                  class="input-field textarea"
                  placeholder="Optional deposit notes..."
              ></textarea>
            </div>

            <!-- Actions -->
            <div class="form-actions">
              <button
                  type="button"
                  class="btn-secondary"
                  @click="closeRecordModal"
              >
                Cancel
              </button>

              <button type="submit" class="btn-primary">
                <i class="fa fa-save"></i>
                Save Record
              </button>
            </div>
          </form>
        </div>
      </div>
    </div>

    <!-- Day Settle Modal -->
    <div v-if="showDaySettleModal" class="modal-overlay" @click="closeDaySettleModal">
      <div class="modal-content" @click.stop>
        <div class="modal-header">
          <h2 v-text="'Day Settlement'"></h2>
          <button class="btn-close" @click="closeDaySettleModal" title="Close">
            <i class="fa fa-times"></i>
          </button>
        </div>

        <div class="modal-body">
          <form @submit.prevent="confirmDaySettle">

            <!-- Day (Uneditable) -->
            <div class="form-group">
              <label for="settlementDay" v-text="'Settlement Date'"></label>
              <span class="required" v-text="'*'"></span>
              <div class="readonly-field">
                <span v-text="formatDate(daySettleForm.date)"></span>
              </div>
            </div>

            <!-- Income Cash -->
            <div class="form-group">
              <label for="incomeCash" v-text="'Income - Cash'"></label>
              <span class="required" v-text="'*'"></span>
              <div class="readonly-field">
                <span v-text="formatPrice(daySettleForm.incomeCash)"></span>
              </div>
            </div>

            <!-- Income Credit -->
            <div class="form-group">
              <label for="incomeCredit" v-text="'Income - Credit'"></label>
              <span class="required" v-text="'*'"></span>
              <div class="readonly-field">
                <span v-text="formatPrice(daySettleForm.incomeCredit)"></span>
              </div>
            </div>

            <!-- Income Bank -->
            <div class="form-group">
              <label for="incomeCredit" v-text="'Income - Bank Transfer Amount'"></label>
              <span class="required" v-text="'*'"></span>
              <div class="readonly-field">
                <span v-text="formatPrice(daySettleForm.incomeBank)"></span>
              </div>
            </div>
            <!-- Expenses Cash -->
            <div class="form-group">
              <label for="expensesCash" v-text="'Expenses - Cash'"></label>
              <span class="required" v-text="'*'"></span>
              <div class="readonly-field">
                <span v-text="formatPrice(daySettleForm.expensesCash)"></span>
              </div>
            </div>

            <!-- Expenses Credit -->
            <div class="form-group">
              <label for="expensesCredit" v-text="'Expenses - Credit'"></label>
              <span class="required" v-text="'*'"></span>
              <div class="readonly-field">
                <span v-text="formatPrice(daySettleForm.expensesCredit)"></span>
              </div>
            </div>

            <!-- Expenses Bank -->
            <div class="form-group">
              <label for="expensesCredit" v-text="'Expenses - Bank/Card Payment'"></label>
              <span class="required" v-text="'*'"></span>
              <div class="readonly-field">
                <span v-text="formatPrice(daySettleForm.expensesBank)"></span>
              </div>
            </div>

            <!-- Bank Deposit -->
            <div class="form-group">
              <label for="bankDeposit" v-text="'Bank Deposit'"></label>
              <span class="required" v-text="'*'"></span>
              <div class="readonly-field">
                <span v-text="formatPrice(daySettleForm.bankDeposit)"></span>
              </div>
            </div>

            <!-- Hands on Cash -->
            <div class="form-group">
              <label for="handsOnCash" v-text="'Cash on Hand'"></label>
              <span class="required" v-text="'*'"></span>
              <div class="readonly-field">
                <span v-text="formatPrice(daySettleForm.handsOnCash)"></span>
              </div>
            </div>

            <!-- Summary Section -->
            <div class="summary-box">
              <div class="summary-row">
                <span class="summary-label" v-text="'Total Income'"></span>
                <span class="summary-value" v-text="formatPrice(daySettleForm.incomeCash + daySettleForm.incomeCredit)"></span>
              </div>
              <div class="summary-row">
                <span class="summary-label" v-text="'Total Expenses'"></span>
                <span class="summary-value" v-text="formatPrice(daySettleForm.expensesCash + daySettleForm.expensesCredit)"></span>
              </div>
              <div class="summary-row final">
                <span class="summary-label" v-text="'Net Balance'"></span>
                <span class="summary-value final" v-text="formatPrice((daySettleForm.incomeCash + daySettleForm.incomeCredit) - (daySettleForm.expensesCash + daySettleForm.expensesCredit))"></span>
              </div>
            </div>

            <!-- Form Actions -->
            <div class="form-actions">
              <button type="button" class="btn-secondary" @click="closeDaySettleModal">
                <i class="fa fa-times"></i>
                <span v-text="'Cancel'"></span>
              </button>
              <button type="submit" class="btn-primary" @click="saveDaySettleRecords">
                <i class="fa fa-check"></i>
                <span v-text="'Confirm & Settle Day'"></span>
              </button>
            </div>
          </form>
        </div>
      </div>
    </div>

    <!-- View Settle Modal -->
    <div v-if="showAccountRecordModal" class="modal-overlay" @click="closeAccountRecordModal">
      <div class="modal-content">
        <div class="modal-header">
          <h2>{{ formatDate(accountRecord.date) }} Account Record </h2>
          <button class="btn-close" @click="closeDaySettleModal" title="Close">
            <i class="fa fa-times"></i>
          </button>
        </div>

        <div class="modal-body">
          <!-- Income Cash -->
          <div class="form-group">
            <label for="incomeCash" v-text="'Income - Cash'"></label>
            <span class="required" v-text="'*'"></span>
            <div class="readonly-field">
              <span v-text="formatPrice(accountRecord.incomeCash)"></span>
            </div>
          </div>

          <!-- Income Credit -->
          <div class="form-group">
            <label for="incomeCredit" v-text="'Income - Credit'"></label>
            <span class="required" v-text="'*'"></span>
            <div class="readonly-field">
              <span v-text="formatPrice(accountRecord.incomeCredit)"></span>
            </div>
          </div>

          <!-- Income Bank -->
          <div class="form-group">
            <label for="incomeCredit" v-text="'Income - Bank Transfer Amount'"></label>
            <span class="required" v-text="'*'"></span>
            <div class="readonly-field">
              <span v-text="formatPrice(accountRecord.incomeBank)"></span>
            </div>
          </div>
          <!-- Expenses Cash -->
          <div class="form-group">
            <label for="expensesCash" v-text="'Expenses - Cash'"></label>
            <span class="required" v-text="'*'"></span>
            <div class="readonly-field">
              <span v-text="formatPrice(accountRecord.expensesCash)"></span>
            </div>
          </div>

          <!-- Expenses Credit -->
          <div class="form-group">
            <label for="expensesCredit" v-text="'Expenses - Credit'"></label>
            <span class="required" v-text="'*'"></span>
            <div class="readonly-field">
              <span v-text="formatPrice(accountRecord.expensesCredit)"></span>
            </div>
          </div>

          <!-- Expenses Bank -->
          <div class="form-group">
            <label for="expensesCredit" v-text="'Expenses - Bank/Card Payment'"></label>
            <span class="required" v-text="'*'"></span>
            <div class="readonly-field">
              <span v-text="formatPrice(accountRecord.expensesBank)"></span>
            </div>
          </div>

          <!-- Bank Deposit -->
          <div class="form-group">
            <label for="bankDeposit" v-text="'Bank Deposit'"></label>
            <span class="required" v-text="'*'"></span>
            <div class="readonly-field">
              <span v-text="formatPrice(accountRecord.bankDeposit)"></span>
            </div>
          </div>

          <!-- Hands on Cash -->
          <div class="form-group">
            <label for="handsOnCash" v-text="'Cash on Hand'"></label>
            <span class="required" v-text="'*'"></span>
            <div class="readonly-field">
              <span v-text="formatPrice(accountRecord.handsOnCash)"></span>
            </div>
          </div>

          <!-- Form Actions -->
          <div class="form-actions">
            <button type="button" class="btn-secondary" @click="closeAccountRecordModal">
              <i class="fa fa-times"></i>
              <span v-text="'Cancel'"></span>
            </button>
          </div>
        </div>
      </div>
    </div>
    <ConfirmationModal ref="confirmDialog"/>
  </div>
</template>

<script setup>
import { ref, reactive, computed, onMounted } from 'vue';
import HeaderComponent from '../component/Header.vue';
import { dbService } from '../services/db.ts';
import { paymentTypes } from "../utils/constants.ts";
import ConfirmationModal from "../component/ConfirmationModal.vue";
import { useI18n } from "vue-i18n";

// Props and Emits
const emit = defineEmits(['logout']);
const { t } = useI18n();
// State
const loggedUser = ref('Admin User');
const records = ref([]);
const maintenance = ref([]);
const expenses = ref([]);
const transactions = ref([]);
const bankDeposit = ref([]);
const lastWeekMaintenance = ref([]);
const lastWeekExpenses = ref([]);
const lastWeekTransactions = ref([]);
const lastWeekBankDeposit = ref([]);

const searchQuery = ref('');
const selectedDate = ref('');
const showRecordModal = ref(false);
const showDaySettleModal = ref(false);
const showAccountRecordModal = ref(false);

const currentDate = ref(new Date().toISOString().split("T")[0]);
const displayDate = ref(new Date().toISOString().split("T")[0]);

const recordForm = reactive({
  type: 'income',
  date: new Date().toISOString().slice(0, 10),
  amount: 0,
  description: '',
  category: '',
  payment_type: '',
  bank_account: '',
  notes: ''
});

const daySettleForm = reactive({
  date: new Date().toISOString().slice(0, 10),
  incomeCash: 0,
  incomeCredit: 0,
  incomeBank: 0,
  expensesCash: 0,
  expensesCredit: 0,
  expensesBank: 0,
  bankDeposit: 0,
  handsOnCash: 0
});

const accountRecord = reactive({
  date: null,
  incomeCash: 0,
  incomeCredit: 0,
  incomeBank: 0,
  expensesCash: 0,
  expensesCredit: 0,
  expensesBank: 0,
  bankDeposit: 0,
  handsOnCash: 0
});

const confirmDialog = ref(null);

// Utility Functions
const formatPrice = (price) => {
  return new Intl.NumberFormat('en-US', {
    style: 'currency',
    currency: 'LKR',
    minimumFractionDigits: 2,
    maximumFractionDigits: 2
  }).format(Number(price || 0));
};

const formatDate = (dateString) => {
  if (!dateString) return '-';

  return new Date(dateString).toLocaleDateString(
      'en-US',
      {
        year: 'numeric',
        month: 'short',
        day: 'numeric'
      }
  );
};

const getDisplayHeadingText = () => {
  if (displayDate.value === currentDate.value) {
    return t('accountPage.headingToday');
  }
  return t('accountPage.heading', { date: formatDate(displayDate.value)});
};

const getDisplayDateText = () => {
  if (displayDate.value === currentDate.value) {
    return t('accountPage.today')
  }
  return formatDate(displayDate.value);
};

const getDateDaysAgo = (days) => {
  const date = new Date();
  date.setDate(date.getDate() - days);
  return date.toISOString().split('T')[0];
};

// Computed Properties - Filter data by displayDate
const totalCashIncome = computed(() => {
  return transactions.value.filter(item => item.payment_type === paymentTypes.cashPayment &&
            new Date(item.created_at).getDate() === new Date(displayDate.value).getDate()
        ).reduce((sum, item) => sum + Number(item.amount || 0), 0);
});

const totalCreditIncome = computed(() => {
  return transactions.value
      .filter(item => item.payment_type === paymentTypes.creditPayment &&
          new Date(item.created_at).getDate() === new Date(displayDate.value).getDate()
      )
      .reduce((sum, item) => sum + Number(item.amount || 0), 0);
});

const totalBankIncome = computed(() => {
  return transactions.value
      .filter(item => item.payment_type === paymentTypes.bankTransfer &&
          new Date(item.created_at).getDate() === new Date(displayDate.value).getDate()
      )
      .reduce((sum, item) => sum + Number(item.amount || 0), 0);
});

const totalCashExpenses = computed(() => {
  const totalExpense = expenses.value
      .filter(item =>
          item.payment_type === paymentTypes.cashPayment &&
          item.date === displayDate.value
      )
      .reduce((sum, item) => sum + Number(item.amount || 0), 0);

  const totalMaintenance = maintenance.value
      .filter(item =>
          item.payment_type === paymentTypes.cashPayment &&
          item.service_date === displayDate.value
      )
      .reduce((sum, item) => sum + Number(item.cost || 0), 0);

  return totalExpense + totalMaintenance;
});

const totalCreditExpenses = computed(() => {
  const totalExpense = expenses.value
      .filter(item =>
          item.payment_type === paymentTypes.creditPayment &&
          item.date === displayDate.value
      )
      .reduce((sum, item) => sum + Number(item.amount || 0), 0);

  const totalMaintenance = maintenance.value
      .filter(item =>
          item.payment_type === paymentTypes.creditPayment &&
          item.service_date === displayDate.value
      )
      .reduce((sum, item) => sum + Number(item.cost || 0), 0);

  return totalExpense + totalMaintenance;
});

const totalBankExpenses = computed(() => {
  const totalExpense = expenses.value
      .filter(item =>
          [paymentTypes.cardPayment, paymentTypes.bankTransfer].includes(item.payment_type) &&
          item.date === displayDate.value
      )
      .reduce((sum, item) => sum + Number(item.amount || 0), 0);

  const totalMaintenance = maintenance.value
      .filter(item =>
          [paymentTypes.cardPayment, paymentTypes.bankTransfer].includes(item.payment_type) &&
          item.service_date === displayDate.value
      )
      .reduce((sum, item) => sum + Number(item.cost || 0), 0);

  return totalExpense + totalMaintenance;
});

const totalDeposits = computed(() => {
  return bankDeposit.value
      .filter(item => item.deposit_date.split('T')[0] === displayDate.value)
      .reduce((sum, item) => sum + Number(item.amount || 0), 0);
});

const totalIncome = computed(() => {
  return totalCashIncome.value + totalCreditIncome.value + totalBankIncome.value;
});

const totalExpenses = computed(() => {
  return totalCreditExpenses.value + totalBankExpenses.value + totalCashExpenses.value;
});

const handOnCash = computed(() => {
  return totalCashIncome.value - totalDeposits.value - totalCashExpenses.value;
});

const canGoToPreviousDate = computed(() => {
  const sevenDaysAgo = getDateDaysAgo(7);
  return displayDate.value > sevenDaysAgo;
});

const canGoToNextDate = computed(() => {
  return displayDate.value < currentDate.value;
});

const filteredDailyRecords = computed(() => {
  let result = records.value;

  if (selectedDate.value) {
    result = result.filter(item => item.date === selectedDate.value);
  }

  if (searchQuery.value) {
    const query = searchQuery.value.toLowerCase();
    result = result.filter(item =>
        Object.values(item).some(value =>
            String(value).toLowerCase().includes(query)
        )
    );
  }

  return result;
});

const disabledDaySettle = computed(() =>{
  return records.value.filter(item => new Date(item.date).getDate() === new Date(displayDate.value).getDate()).length === 0 || haveDifference.value.totalIncomeDifference || haveDifference.value.totalExpenseDifference;
});

const isDaySettle = computed(() =>{
  return records.value.filter(item => new Date(item.date).getDate() === new Date(displayDate.value).getDate()).length !== 0
})

// Get saved values from account summary table
const savedSummary = computed(() => {
  const savedRecord = records.value.find(record => record.date === displayDate.value);
  const savedIncomeCash = savedRecord?.income_cash || 0
  const savedIncomeCredit = savedRecord?.income_credit || 0
  const savedIncomeBank = savedRecord?.income_bank || 0
  const savedExpensesCash = savedRecord?.expense_cash || 0
  const savedExpensesCredit = savedRecord?.expense_credit || 0
  const savedExpensesBank = savedRecord?.expense_bank || 0
  const savedHandsOnCash = savedRecord?.cash_amount || 0
  const savedBankDeposit = savedRecord?.deposit_amount || 0
  const savedTotalIncome = savedIncomeBank + savedIncomeCredit + savedIncomeCash
  const savedExpense = savedExpensesCash + savedExpensesCredit + savedExpensesBank

  return { savedTotalIncome, savedExpense, savedIncomeCash, savedIncomeCredit, savedIncomeBank,
    savedExpensesBank, savedExpensesCredit, savedExpensesCash, savedHandsOnCash, savedBankDeposit}
});

const haveDifference = computed(() => {
  const totalIncomeDifference = isDaySettle ? totalIncome.value - savedSummary.value.savedTotalIncome !== 0 : false
  const totalExpenseDifference = isDaySettle ? totalExpenses.value - savedSummary.value.savedExpense !== 0 : false
  const totalIncomeCashDifference = isDaySettle ?  totalCashIncome.value - savedSummary.value.savedIncomeCash !== 0 : false
  const totalIncomeCreditDifference = isDaySettle ?  totalCreditIncome.value - savedSummary.value.savedIncomeCredit !== 0 : false
  const totalIncomeBankDifference = isDaySettle ?  totalBankIncome.value - savedSummary.value.savedIncomeBank !== 0 : false
  const totalExpensesCashDifference = isDaySettle ?  totalCashExpenses.value - savedSummary.value.savedExpensesCash !== 0 : false
  const totalExpensesBankDifference = isDaySettle ?  totalBankExpenses.value - savedSummary.value.savedExpensesBank !== 0 : false
  const totalExpensesCreditDifference = isDaySettle ?  totalCreditExpenses.value - savedSummary.value.savedExpensesCredit !== 0 : false
  const totalHandsOnCashDifference = isDaySettle ?  handOnCash.value - savedSummary.value.savedHandsOnCash !== 0 : false
  const totalBankDepositDifference = isDaySettle ?  totalDeposits.value - savedSummary.value.savedBankDeposit !== 0 : false

  return {totalIncomeDifference, totalExpenseDifference,totalIncomeCashDifference, totalIncomeCreditDifference, totalIncomeBankDifference,
  totalExpensesCashDifference, totalExpensesBankDifference, totalHandsOnCashDifference, totalExpensesCreditDifference, totalBankDepositDifference}
});

// Methods
const previousDate = () => {
  const date = new Date(displayDate.value);
  date.setDate(date.getDate() - 1);
  displayDate.value = date.toISOString().split('T')[0];
};

const nextDate = () => {
  const date = new Date(displayDate.value);
  date.setDate(date.getDate() + 1);
  displayDate.value = date.toISOString().split('T')[0];
};

const openAddRecordModal = (type = 'income') => {
  recordForm.type = type;
  recordForm.date = new Date().toISOString().slice(0, 10);
  recordForm.amount = 0;
  recordForm.description = '';
  recordForm.category = '';
  recordForm.payment_type = '';
  recordForm.bank_account = '';
  recordForm.notes = '';

  showRecordModal.value = true;
};

const closeRecordModal = () => {
  showRecordModal.value = false;
};

const closeAccountRecordModal = () => {
  showAccountRecordModal.value = false;
};

const closeDaySettleModal = () => {
  showDaySettleModal.value = false;
};

const saveRecord = async () => {
  if (!recordForm.amount || !recordForm.date) {
    alert('Please enter date and amount.');
    return;
  }

  const record = {
    deposit_date: recordForm.date,
    amount: Number(recordForm.amount),
    bank_name: recordForm.bank_account,
    notes: recordForm.notes,
    created_by: 'ADMIN'
  };

  try {
    await dbService.addBankRecords(record);
    alert('Record added successfully.');
    closeRecordModal();
    await getBankRecords();
  } catch (error) {
    console.error('Error saving account record:', error);
    alert('Error saving record. Please check the account database/service.');
  }
};

const viewRecord = (record) => {
  accountRecord.date = record.date;
  accountRecord.incomeCash = record.income_cash;
  accountRecord.incomeCredit = record.income_credit;
  accountRecord.incomeBank = record.income_bank;
  accountRecord.expensesCash = record.expense_cash;
  accountRecord.expensesCredit = record.expense_credit;
  accountRecord.expensesBank = record.expense_bank;
  accountRecord.bankDeposit = record.deposit_amount;
  accountRecord.handsOnCash = record.cash_amount;

  showAccountRecordModal.value = true;
};

const deleteRecord = (id) => {
  if (!confirm('Are you sure you want to delete this record?')) {
    return;
  }
  records.value = records.value.filter(item => item.id !== id);
};

const handleLogout = () => {
  emit('logout');
};

const openDaySettleModal = () => {
  daySettleForm.date = displayDate.value;
  daySettleForm.incomeCash = totalCashIncome.value;
  daySettleForm.incomeCredit = totalCreditIncome.value;
  daySettleForm.incomeBank = totalBankIncome.value;
  daySettleForm.expensesCash = totalCashExpenses.value;
  daySettleForm.expensesCredit = totalCreditExpenses.value;
  daySettleForm.expensesBank = totalBankExpenses.value;
  daySettleForm.bankDeposit = totalDeposits.value;
  daySettleForm.handsOnCash = handOnCash.value;

  showDaySettleModal.value = true;
};

const getAccountRecords = async () => {
  try {
    const accountRecords = await dbService.getAccountSummery();
    records.value = accountRecords.map(item => ({
      id: item.date,
      date: item.date,
      expense_cash: item.expenses_cash,
      expense_credit: item.expenses_credit,
      expense_bank: item.expenses_bank,
      income_cash: item.income_cash,
      income_credit: item.income_credit,
      income_bank: item.income_bank_transfer,
      deposit_amount: item.bank_deposit,
      cash_amount: item.hand_on_cash,
    }));
  } catch (error) {
    console.error('Error fetching account records:', error);
    records.value = [];
  }
};

const getExpenseRecords = async () => {
  try {
    lastWeekExpenses.value = await dbService.getExpenses();
    expenses.value = lastWeekExpenses.value;
  } catch (error) {
    console.error('Error fetching expense records:', error);
    expenses.value = [];
  }
};

const getMaintenanceRecords = async () => {
  try {
    lastWeekMaintenance.value = await dbService.getMaintenanceRecords();
    maintenance.value = lastWeekMaintenance.value;
  } catch (error) {
    console.error('Error fetching maintenance records:', error);
    maintenance.value = [];
  }
};

const getTransactions = async () => {
  try {
    lastWeekTransactions.value = await dbService.getTransactions();
    transactions.value = lastWeekTransactions.value;
  } catch (error) {
    console.error('Error fetching transactions:', error);
    transactions.value = [];
  }
};

const getBankRecords = async () => {
  try {
    lastWeekBankDeposit.value = await dbService.getBankRecords();
    bankDeposit.value = lastWeekBankDeposit.value;
  } catch (error) {
    console.error('Error fetching bank records:', error);
    bankDeposit.value = [];
  }
};

const saveDaySettleRecords = async () => {
  confirmDialog.value.open({
    title: 'Day Settle',
    subtitle: 'This cannot be modified or redo',
    message: 'Are you sure you want to settle the day?',
    type: 'info',
    confirmText: 'Yes',
    onConfirm: async () => {
      await saveSettleRecords();
    },
  });
};

const saveSettleRecords = async () => {
  const record = {
    date: daySettleForm.date,
    income_cash: daySettleForm.incomeCash,
    income_credit: daySettleForm.incomeCredit,
    income_bank_transfer: daySettleForm.incomeBank,
    expenses_cash: daySettleForm.expensesCash,
    expenses_credit: daySettleForm.expensesCredit,
    expenses_bank: daySettleForm.expensesBank,
    bank_deposit: daySettleForm.bankDeposit,
    hand_on_cash: daySettleForm.handsOnCash,
    created_by: "ADMIN"
  };

  try {
    await dbService.addAccountSummery(record);
    await getAccountRecords();
    closeDaySettleModal();
    alert('Day settled successfully.');
  } catch (error) {
    console.error('Error occurred on adding account summery:', error);
    alert('Error settling the day. Please try again.');
  }
};

// Lifecycle
onMounted(async () => {
  await getAccountRecords();
  await getExpenseRecords();
  await getMaintenanceRecords();
  await getTransactions();
  await getBankRecords();
});
</script>

<style scoped>
.account-page {
  display: flex;
  flex-direction: column;
  height: 100vh;
  overflow: hidden;
  background: var(--color-background-tertiary);
}

.account-content {
  flex: 1;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.account-container {
  flex: 1;
  overflow-y: auto;
  padding: 1.5rem;
  max-width: 1500px;
  margin: 0 auto;
  width: 100%;
  box-sizing: border-box;
}

/* Header */
.page-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  gap: 1rem;
  margin-bottom: 1.5rem;
}

.header-title {
  flex: 1;
}

.date-navigation {
  display: flex;
  align-items: center;
  gap: 1rem;
  margin-bottom: 0.5rem;
}

.btn-date-nav {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 40px;
  height: 40px;
  border: 1px solid var(--color-border-tertiary);
  background: var(--color-background-secondary);
  border-radius: 8px;
  cursor: pointer;
  color: var(--color-text-primary);
  font-size: 16px;
  transition: all 0.2s ease;
  flex-shrink: 0;
}

.btn-date-nav:hover:not(:disabled) {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border-color: transparent;
}

.btn-date-nav:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.btn-prev {
  order: -1;
}

.page-header h1 {
  margin: 0;
  font-size: 28px;
  font-weight: 700;
  color: var(--color-text-primary);
}

.page-subtitle {
  margin: 0.35rem 0 0;
  font-size: 13px;
  color: var(--color-text-secondary);
}

.header-actions {
  display: flex;
  gap: 0.6rem;
  flex-wrap: wrap;
}

.btn-action-primary,
.btn-action-danger,
.btn-action-bank {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 0.5rem;
  padding: 0.7rem 1rem;
  border: none;
  border-radius: var(--border-radius-md);
  cursor: pointer;
  font-size: 13px;
  font-weight: 600;
  color: white;
}

.btn-action-primary {
  background: #16a34a;
}

.btn-action-danger {
  background: #dc2626;
}

.btn-action-bank {
  background: #2563eb;
}

/* Stats */
.stats-section {
  display: grid;
  grid-template-columns: repeat(5, minmax(0, 1fr));
  gap: 1rem;
  margin-bottom: 1.5rem;
}

.stat-card {
  position: relative;
  background: var(--color-background-primary);
  border: 1px solid var(--color-border-tertiary);
  border-radius: var(--border-radius-lg);
  padding: 1.15rem;
  display: flex;
  align-items: center;
  gap: 1rem;
  min-width: 0;
}

.stat-icon {
  width: 44px;
  height: 44px;
  border-radius: 12px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--color-background-secondary);
  flex-shrink: 0;
}

.stat-previous {
  display: block;
  font-size: 11px;
  color: #ef1515;
  margin-top: 0.3rem;
  font-weight: 1000;
}

.mismatch-badge {
  position: absolute;
  top: -8px;
  right: -8px;
  width: 28px;
  height: 28px;
  background: #dc2626;
  color: white;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: 700;
  font-size: 16px;
  box-shadow: 0 2px 4px rgba(220, 38, 38, 0.3);
}

.stat-card.card-mismatch {
  border-color: #f59e0b;
  background: linear-gradient(135deg, var(--color-background-primary) 0%, #fffbeb 100%);
  box-shadow: 0 0 0 2px rgba(245, 158, 11, 0.1);
}

.cash-balance-card .stat-icon {
  color: #16a34a;
}

.credit-balance-card .stat-icon {
  color: #7c3aed;
}

.income-card .stat-icon {
  color: var(--color-success);
}

.expense-card .stat-icon {
  color: var(--color-danger);
}

.deposit-card .stat-icon {
  color: #2563eb;
}

.net-card .stat-icon {
  color: #0891b2;
}

.stat-card label {
  display: block;
  font-size: 11px;
  color: var(--color-text-secondary);
  font-weight: 600;
  margin-bottom: 0.35rem;
}

.stat-value {
  display: block;
  font-size: 18px;
  font-weight: 700;
  color: var(--color-text-primary);
  white-space: nowrap;
}

/* Table */
.table-section {
  background: var(--color-background-primary);
  border-radius: var(--border-radius-lg);
  border: 1px solid var(--color-border-tertiary);
  overflow: hidden;
}

.table-header {
  padding: 1.25rem 1.5rem;
  border-bottom: 1px solid var(--color-border-tertiary);
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 1rem;
}

.section-title {
  margin: 0;
  font-size: 16px;
  color: var(--color-text-primary);
}

.table-subtitle {
  margin: 0.3rem 0 0;
  font-size: 12px;
  color: var(--color-text-secondary);
}

.table-filters {
  display: flex;
  gap: 0.6rem;
  width: 420px;
}

.search-input,
.date-filter {
  box-sizing: border-box;
  padding: 0.65rem 0.8rem;
  border: 1px solid var(--color-border-tertiary);
  border-radius: 6px;
  font-size: 13px;
  color: var(--color-text-primary);
  background: var(--color-background-secondary);
}

.search-input {
  width: 100%;
}

.date-filter {
  width: 145px;
}

.search-input:focus,
.date-filter:focus,
.input-field:focus {
  outline: none;
  border-color: var(--color-info);
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
}

.table-wrapper {
  overflow-x: auto;
}

.account-table {
  width: 100%;
  border-collapse: collapse;
  font-size: 13px;
  min-width: 950px;
}

.account-table th {
  padding: 0.75rem;
  text-align: center;
  color: var(--color-text-secondary);
  background: var(--color-background-secondary);
  font-weight: 600;
  border-bottom: 1px solid var(--color-border-tertiary);
}

.account-table td {
  padding: 0.85rem 0.75rem;
  border-bottom: 1px solid var(--color-border-tertiary);
  color: var(--color-text-primary);
  text-align: right;
}

.account-table td:first-child {
  text-align: left;
}

.group-expense {
  color: var(--color-danger) !important;
}

.group-income {
  color: var(--color-success) !important;
}

.group-deposit {
  color: #2563eb !important;
}

.date-cell {
  font-weight: 600;
  white-space: nowrap;
}

.amount-expense,
.amount-credit-expense {
  color: var(--color-danger) !important;
  font-family: monospace;
}

.amount-income,
.amount-credit-income {
  color: var(--color-success) !important;
  font-family: monospace;
  text-align: left;
}

.amount-deposit {
  color: #2563eb !important;
  font-family: monospace;
  font-weight: 700;
}

.amount-cash {
  color: #0891b2 !important;
  font-family: monospace;
  font-weight: 700;
}

.actions {
  display: flex;
  justify-content: center;
  gap: 0.4rem;
}

.btn-action {
  padding: 0.4rem 0.5rem;
  border: 1px solid var(--color-border-tertiary);
  background: transparent;
  color: var(--color-text-secondary);
  border-radius: 4px;
  cursor: pointer;
}

.btn-action:hover {
  color: var(--color-info);
  border-color: var(--color-info);
}

/* Empty */
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 3rem;
  color: var(--color-text-secondary);
}

.empty-state i {
  font-size: 48px;
  margin-bottom: 1rem;
  opacity: 0.3;
}

.btn-add-empty {
  padding: 0.7rem 1.25rem;
  border: none;
  border-radius: var(--border-radius-md);
  background: linear-gradient(
      135deg,
      #667eea 0%,
      #764ba2 100%
  );
  color: white;
  cursor: pointer;
  font-weight: 600;
}

/* Modal */
.modal-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  padding: 1rem;
}

.modal-content {
  background: var(--color-background-primary);
  border-radius: var(--border-radius-lg);
  width: 90%;
  max-width: 650px;
  max-height: 90vh;
  overflow-y: auto;
  box-shadow: 0 10px 40px rgba(0, 0, 0, 0.2);
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1.25rem 1.5rem;
  background: linear-gradient(
      135deg,
      #667eea 0%,
      #764ba2 100%
  );
  color: white;
  border-radius:
      var(--border-radius-lg)
      var(--border-radius-lg)
      0
      0;
}

.modal-header h2 {
  margin: 0;
  font-size: 19px;
}

.modal-subtitle {
  margin: 0.25rem 0 0;
  font-size: 12px;
  opacity: 0.85;
}

.btn-close {
  border: none;
  background: transparent;
  color: white;
  cursor: pointer;
  font-size: 19px;
}

.modal-body {
  padding: 1.5rem;
}

.form-group {
  margin-bottom: 1.25rem;
}

.form-row {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 1rem;
}

.form-group label {
  display: block;
  margin-bottom: 0.55rem;
  font-size: 13px;
  font-weight: 600;
  color: var(--color-text-primary);
}

.required {
  color: var(--color-danger);
}

.input-field {
  width: 100%;
  box-sizing: border-box;
  padding: 0.75rem;
  border: 1px solid var(--color-border-tertiary);
  border-radius: 5px;
  font-size: 14px;
  color: var(--color-text-primary);
  background: var(--color-background-primary);
}

.textarea {
  resize: vertical;
  min-height: 80px;
}

/* Record Type */
.record-type-selector {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 0.6rem;
}

.type-button {
  padding: 0.75rem;
  border: 1px solid var(--color-border-tertiary);
  background: var(--color-background-secondary);
  color: var(--color-text-secondary);
  border-radius: 6px;
  cursor: pointer;
  font-weight: 600;
}

.type-button:hover {
  border-color: var(--color-info);
}

.type-button.active {
  background: var(--color-info);
  color: white;
  border-color: var(--color-info);
}

/* Deposit */
.deposit-info {
  display: flex;
  gap: 0.7rem;
  align-items: flex-start;
  padding: 0.85rem;
  margin-bottom: 1.25rem;
  border-radius: 6px;
  background: var(--color-background-secondary);
  color: var(--color-text-secondary);
  font-size: 12px;
}

.deposit-info i {
  color: #2563eb;
  margin-top: 2px;
}

.deposit-preview {
  display: grid;
  grid-template-columns: 1fr auto 1fr;
  align-items: center;
  gap: 1rem;
  padding: 1rem;
  margin-bottom: 1.25rem;
  border: 1px solid var(--color-border-tertiary);
  border-radius: 8px;
  background: var(--color-background-secondary);
}

.deposit-preview div:not(.arrow) {
  display: flex;
  flex-direction: column;
  gap: 0.3rem;
}

.deposit-preview span {
  font-size: 11px;
  color: var(--color-text-secondary);
}

.deposit-preview strong {
  font-size: 15px;
  color: var(--color-text-primary);
}

.arrow {
  color: #2563eb;
}

/* Readonly Fields */
.readonly-field {
  padding: 0.75rem;
  background: var(--color-background-secondary);
  border: 1px solid var(--color-border-tertiary);
  border-radius: 5px;
  font-size: 14px;
  color: var(--color-text-primary);
  display: flex;
  align-items: center;
}

.readonly-field span {
  font-family: monospace;
  font-weight: 600;
}

/* Summary Box */
.summary-box {
  background: var(--color-background-secondary);
  border: 1px solid var(--color-border-tertiary);
  border-radius: 8px;
  padding: 1.25rem;
  margin-bottom: 1.25rem;
  margin-top: 1.25rem;
}

.summary-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.75rem 0;
  border-bottom: 1px solid var(--color-border-tertiary);
}

.summary-row:last-child {
  border-bottom: none;
}

.summary-row.final {
  background: var(--color-background-primary);
  padding: 0.85rem 0.75rem;
  margin: 0.5rem -0.75rem -0.75rem;
  border-radius: 0 0 8px 8px;
  border-top: 2px solid var(--color-border-tertiary);
  border-bottom: none;
}

.summary-label {
  font-size: 13px;
  font-weight: 600;
  color: var(--color-text-secondary);
}

.summary-value {
  font-size: 14px;
  font-weight: 700;
  color: var(--color-text-primary);
  font-family: monospace;
}

.summary-value.final {
  color: #0891b2;
  font-size: 16px;
}

/* Form buttons */
.form-actions {
  display: flex;
  justify-content: flex-end;
  gap: 0.75rem;
  padding-top: 1.25rem;
  border-top: 1px solid var(--color-border-tertiary);
}

.btn-primary {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.75rem 1.25rem;
  border: none;
  border-radius: var(--border-radius-md);
  background: linear-gradient(
      135deg,
      #667eea 0%,
      #764ba2 100%
  );
  color: white;
  cursor: pointer;
  font-weight: 600;
}

.btn-secondary {
  padding: 0.75rem 1.25rem;
  border: 1px solid var(--color-border-tertiary);
  background: transparent;
  color: var(--color-text-primary);
  border-radius: var(--border-radius-md);
  cursor: pointer;
  font-weight: 600;
}

/* Responsive */
@media (max-width: 1200px) {
  .stats-section {
    grid-template-columns: repeat(3, 1fr);
  }
}

@media (max-width: 1024px) {
  .page-header {
    flex-direction: column;
  }

  .header-actions {
    width: 100%;
  }

  .header-actions button {
    flex: 1;
  }

  .stats-section {
    grid-template-columns: repeat(2, 1fr);
  }

  .table-header {
    align-items: flex-start;
    flex-direction: column;
  }

  .table-filters {
    width: 100%;
  }
}

@media (max-width: 768px) {
  .account-container {
    padding: 1rem;
  }

  .page-header h1 {
    font-size: 24px;
  }

  .date-navigation {
    gap: 0.75rem;
  }

  .btn-date-nav {
    width: 36px;
    height: 36px;
  }

  .page-header h1 {
    font-size: 22px;
  }

  .header-actions {
    flex-direction: column;
  }

  .header-actions button {
    width: 100%;
  }

  .stats-section {
    grid-template-columns: 1fr;
  }

  .table-filters {
    flex-direction: column;
  }

  .date-filter {
    width: 100%;
  }

  .form-row {
    grid-template-columns: 1fr;
  }

  .record-type-selector {
    grid-template-columns: 1fr;
  }

  .deposit-preview {
    grid-template-columns: 1fr;
  }

  .deposit-preview .arrow {
    transform: rotate(90deg);
    text-align: center;
  }

  .form-actions {
    flex-direction: column;
  }

  .form-actions button {
    width: 100%;
  }
}
</style>
