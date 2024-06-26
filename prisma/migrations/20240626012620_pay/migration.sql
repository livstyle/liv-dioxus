-- CreateTable
CREATE TABLE "PaymentRecord" (
    "id" BIGSERIAL NOT NULL,
    "amount" BIGINT NOT NULL DEFAULT 0,

    CONSTRAINT "PaymentRecord_pkey" PRIMARY KEY ("id")
);
