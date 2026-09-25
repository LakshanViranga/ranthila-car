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
        label: 'By Mileage',
        value: 'MILEAGE',
    },
    {
        label: 'By Date',
        value: 'DATE',
    },
    {
        label: 'Both',
        value: 'BOTH',
    },
    {
        label: 'None',
        value: 'NONE',
    }
]

export const nextServiceTypeConst = {
    fromMileage: 'MILEAGE',
    fromDate: 'DATE',
    fromBoth: 'BOTH',
    none: 'NONE',
}

export const nextServiceMileage = [
    {
        label: '5000KM',
        value: 5000,
    },
    {
        label: '7500KM',
        value: 7500,
    },
    {
        label: '10000KM',
        value: 10000,
    },
    {
        label: '20000KM',
        value: 20000,
    },
    {
        label: '40000KM',
        value: 40000,
    }
]

export const maintenanceTypes = {
    vehicleInsurance: 'VEHICLE_INSURANCE',
    revenueInsurance: 'REVENUE_INSURANCE',
    fullService: 'FULL_SERVICE',
    acRepair: 'AC_REPAIR',
    generalRepair: 'GENERAL_REPAIR',
    bodyWash: 'BODY_WASH',
    accidentRepair: 'ACCIDENT_REPAIR',
}

export const maintenanceTypeArray = [
    {
        label: 'Vehicle Insurance renew',
        value: 'VEHICLE_INSURANCE'
    },
    {
        label: 'Revenue Licence renew',
        value: 'REVENUE_LICENCE'
    },
    {
        label: 'Full Service',
        value: 'FULL_SERVICE',
    },
    {
        label: 'AC Repair',
        value: 'AC_REPAIR',
    },
    {
        label: 'Genera Repair',
        value: 'GENERAL_REPAIR',
    },
    {
        label: 'Body wash',
        value: 'BODY_WASH'
    },
    {
        label: 'Accident Repair',
        value: 'ACCIDENT_REPAIR'
    }
]

export const getLabelValues = (inputArray, value) => {
    return inputArray.find((item) => item.value === value )?.label || 'Unknown'
}
export const convertSnakeCase = (input: string) => {
   return input.replaceAll(" ", "_").toLowerCase()
}
