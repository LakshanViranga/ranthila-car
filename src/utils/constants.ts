export const paymentTypes = {
    cashPayment: 'CASH',
    creditPayment: 'CREDIT',
    bankTransfer: 'BANK_TRANSFER',
    cardPayment: 'CARD_PAYMENT'
}

export const orderStatus = {
    reserved: 'RESERVED',
    active: 'ACTIVE',
    completed: 'COMPLETED',
}

export const paymentStatus = {
    notPaid: 'NOT PAID',
    partialPaid: 'PARTIAL PAID',
    paid: 'PAID',
    completed: 'COMPLETED',
    pending: 'PENDING',
}

export const transmissionType = {
    automatic: 'AUTOMATIC',
    manual: 'MANUAL',
}

export const expensesType = {
    utility: 'UTILITY',
    salary: 'SALARY',
}

const maintenanceType = {
    vehicleInsurance: 'VEHICLE_INSURANCE',
    revenueInsurance: 'REVENUE_INSURANCE',
    fullService: 'FULLY_SERVICE',
}

export const userStatus = {
    ACTIVE: 'ACTIVE',
    INACTIVE: 'INACTIVE',
}

export const roleTypes = {
    admin: 'ADMIN',
    staff: 'STAFF',
}

export const guaranteePropertyType = [
    {
        title: 'Own Vehicle',
        value: 'OWN_VEHICLE',
    },
    {
        title: 'Other Vehicle',
        value: 'OTHER_VEHICLE',
    },
    {
        title: 'Cash Deposit',
        value: 'CASH_DEPOSIT',
    }
]

export const paymentTypeArray = [
    {
        title: 'Cash',
        value: 'CASH',
    },
    {
        title: 'Credit',
        value: 'CREDIT',
    },
    {
        title: 'Bank Transfer',
        value: 'BANK_TRANSFER',
    },
    {
        title: 'Card Payment',
        value: 'CARD_PAYMENT',
    }
]

export const nextServiceType = [
    {
        title: 'By Mileage',
        value: 'MILEAGE',
    },
    {
        title: 'By Date',
        value: 'DATE',
    },
    {
        title: 'Both',
        value: 'BOTH',
    },
    {
        title: 'None',
        value: 'NONE',
    }
]

export const convertSnakeCase = (input: string) => {
   return input.replaceAll(" ", "_").toLowerCase()
}
