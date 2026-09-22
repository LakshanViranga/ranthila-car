import { reactive } from 'vue';

const snackbar = reactive({
    show: false,
    message: '',
    type: 'success'
});

const showSuccess = (message) => {
    snackbar.message = message;
    snackbar.type = 'success';
    snackbar.show = true;
};

const showError = (message) => {
    snackbar.message = message;
    snackbar.type = 'error';
    snackbar.show = true;
};

export const useSnackbar = () => ({
    snackbar,
    showSuccess,
    showError
});
