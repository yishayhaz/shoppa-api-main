pub struct UsersFields {
    pub id: &'static str,
    pub created_at: &'static str,
    pub updated_at: &'static str,
    pub level: &'static str,
    pub name: &'static str,
    pub email: &'static str,
    pub phone_number: &'static str,
    pub password: &'static str,
    pub gender: &'static str,
    pub date_of_birth: &'static str,
    pub address: &'static str,
    pub credit_cards: &'static str,
    pub cart: &'static str,
}

pub struct CartFields {
    pub items: &'static str,
    pub coupon: &'static str,
    pub total_price: &'static str,
}

pub struct CartItemsFields {
    pub added_at: &'static str,
    pub product: &'static str,
    pub quantity: &'static str,
}

pub struct AddressFields {
    pub id: &'static str,
    pub name: &'static str,
    pub default: &'static str,
    pub deleted: &'static str,
}

pub struct CreditCardFieds {
    pub id: &'static str,
    pub name: &'static str,
    pub default: &'static str,
    pub deleted: &'static str,
}

impl UsersFields {
    pub fn cart(&self) -> &CartFields {
        &CART_FIELDS
    }

    pub fn address(&self) -> &AddressFields {
        &ADDRESS_FIELDS
    }

    pub fn credit_cards(&self) -> &CreditCardFieds {
        &CREDIT_CARD_FIELDS
    }
}

impl CartFields {
    pub fn items(&self) -> &CartItemsFields {
        &CART_ITEMS_FIELDS
    }
}

pub const FIELDS: UsersFields = UsersFields {
    id: "_id",
    created_at: "created_at",
    updated_at: "updated_at",
    level: "level",
    name: "name",
    email: "email",
    phone_number: "phone_number",
    password: "password",
    gender: "gender",
    date_of_birth: "date_of_birth",
    address: "address",
    credit_cards: "credit_cards",
    cart: "cart",
};

const ADDRESS_FIELDS: AddressFields = AddressFields {
    id: "_id",
    name: "name",
    default: "default",
    deleted: "deleted",
};

const CREDIT_CARD_FIELDS: CreditCardFieds = CreditCardFieds {
    id: "_id",
    name: "name",
    default: "default",
    deleted: "deleted",
};

const CART_FIELDS: CartFields = CartFields {
    items: "items",
    coupon: "coupon",
    total_price: "total_price",
};

const CART_ITEMS_FIELDS: CartItemsFields = CartItemsFields {
    added_at: "added_at",
    product: "product",
    quantity: "quantity",
};
