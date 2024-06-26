-- CreateTable
CREATE TABLE "system_users" (
    "id" BIGSERIAL NOT NULL,
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
    "deleted" BOOLEAN NOT NULL DEFAULT false,
    "deleted_at" TIMESTAMP(3),

    CONSTRAINT "system_users_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "system_roles" (
    "id" SERIAL NOT NULL,
    "name" TEXT NOT NULL,
    "describe" TEXT NOT NULL DEFAULT '',
    "status" INTEGER NOT NULL DEFAULT 0,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP(3) NOT NULL,
    "deleted_at" TIMESTAMP(3),
    "deleted" BOOLEAN NOT NULL DEFAULT false,

    CONSTRAINT "system_roles_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "system_depts" (
    "id" SERIAL NOT NULL,
    "name" TEXT NOT NULL,
    "describe" TEXT NOT NULL DEFAULT '',
    "status" INTEGER NOT NULL DEFAULT 0,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP(3) NOT NULL,
    "deleted_at" TIMESTAMP(3),
    "deleted" BOOLEAN NOT NULL DEFAULT false,

    CONSTRAINT "system_depts_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "system_menus" (
    "id" SERIAL NOT NULL,
    "name" TEXT NOT NULL,
    "icon" TEXT NOT NULL DEFAULT '',
    "path" TEXT NOT NULL DEFAULT '',
    "component" TEXT NOT NULL DEFAULT '',
    "status" INTEGER NOT NULL DEFAULT 0,
    "deleted" BOOLEAN NOT NULL DEFAULT false,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP(3) NOT NULL,
    "deleted_at" TIMESTAMP(3),

    CONSTRAINT "system_menus_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "system_brands" (
    "id" BIGSERIAL NOT NULL,
    "name" TEXT NOT NULL,
    "en_name" TEXT NOT NULL DEFAULT '',
    "describe" TEXT NOT NULL DEFAULT '',
    "status" INTEGER NOT NULL DEFAULT 0,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP(3) NOT NULL,
    "deleted_at" TIMESTAMP(3),
    "deleted" BOOLEAN NOT NULL DEFAULT false,
    "logo" TEXT,
    "sort" INTEGER,
    "brand_type" TEXT,

    CONSTRAINT "system_brands_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "goods_categories" (
    "id" BIGSERIAL NOT NULL,
    "name" TEXT NOT NULL,
    "describe" TEXT NOT NULL DEFAULT '',
    "status" INTEGER NOT NULL DEFAULT 0,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP(3) NOT NULL,
    "deleted_at" TIMESTAMP(3),
    "deleted" BOOLEAN NOT NULL DEFAULT false,
    "sort" INTEGER,
    "category_type" TEXT,
    "level" INTEGER,
    "parent_id" INTEGER,
    "parent_ids" TEXT,
    "path" TEXT,

    CONSTRAINT "goods_categories_pkey" PRIMARY KEY ("id")
);

-- CreateIndex
CREATE UNIQUE INDEX "system_users_username_key" ON "system_users"("username");

-- CreateIndex
CREATE UNIQUE INDEX "system_roles_name_key" ON "system_roles"("name");

-- CreateIndex
CREATE UNIQUE INDEX "system_depts_name_key" ON "system_depts"("name");

-- CreateIndex
CREATE UNIQUE INDEX "system_menus_name_key" ON "system_menus"("name");

-- CreateIndex
CREATE UNIQUE INDEX "system_brands_name_key" ON "system_brands"("name");

-- CreateIndex
CREATE UNIQUE INDEX "goods_categories_name_key" ON "goods_categories"("name");
