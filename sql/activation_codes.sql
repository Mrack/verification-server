/*
 Navicat Premium Data Transfer

 Source Server         : 192.168.120.137
 Source Server Type    : PostgreSQL
 Source Server Version : 150002 (150002)
 Source Host           : 192.168.120.137:5432
 Source Catalog        : registration_system
 Source Schema         : public

 Target Server Type    : PostgreSQL
 Target Server Version : 150002 (150002)
 File Encoding         : 65001

 Date: 12/05/2023 10:45:02
*/


-- ----------------------------
-- Table structure for activation_codes
-- ----------------------------
DROP TABLE IF EXISTS "public"."activation_codes";
CREATE TABLE "public"."activation_codes" (
  "id" int4 NOT NULL DEFAULT nextval('activation_codes_id_seq'::regclass),
  "code" text COLLATE "pg_catalog"."default" NOT NULL,
  "device_id" text COLLATE "pg_catalog"."default",
  "used" bool NOT NULL DEFAULT false,
  "activated_at" timestamp(6),
  "end_hour" int8 NOT NULL
)
;

-- ----------------------------
-- Uniques structure for table activation_codes
-- ----------------------------
ALTER TABLE "public"."activation_codes" ADD CONSTRAINT "activation_codes_code_key" UNIQUE ("code");

-- ----------------------------
-- Primary Key structure for table activation_codes
-- ----------------------------
ALTER TABLE "public"."activation_codes" ADD CONSTRAINT "activation_codes_pkey" PRIMARY KEY ("id");

-- ----------------------------
-- Foreign Keys structure for table activation_codes
-- ----------------------------
ALTER TABLE "public"."activation_codes" ADD CONSTRAINT "activation_codes_device_id_fkey" FOREIGN KEY ("device_id") REFERENCES "public"."devices" ("id") ON DELETE NO ACTION ON UPDATE NO ACTION;
