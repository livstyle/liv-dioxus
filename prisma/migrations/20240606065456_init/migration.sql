-- CreateTable
CREATE TABLE "SystemUser" (
    "id" SERIAL NOT NULL,
    "username" TEXT NOT NULL,
    "nickname" TEXT NOT NULL DEFAULT '',
    "role_id" INTEGER,
    "dept_id" INTEGER,
    "phone" TEXT NOT NULL DEFAULT '',
    "email" TEXT NOT NULL DEFAULT '',
    "sex" INTEGER NOT NULL DEFAULT 1,
    "password" TEXT NOT NULL DEFAULT '',
    "salt" TEXT NOT NULL DEFAULT '',
    "describe" TEXT NOT NULL DEFAULT '',
    "expire_time" TIMESTAMP(3),
    "status" INTEGER NOT NULL DEFAULT 0,
    "last_login_ip" TEXT NOT NULL DEFAULT '',
    "last_login_time" TIMESTAMP(3),
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP(3) NOT NULL,
    "deleted_at" TIMESTAMP(3),

    CONSTRAINT "SystemUser_pkey" PRIMARY KEY ("id")
);

-- CreateIndex
CREATE UNIQUE INDEX "SystemUser_username_key" ON "SystemUser"("username");
