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

 Date: 12/05/2023 10:45:09
*/


-- ----------------------------
-- Table structure for devices
-- ----------------------------
DROP TABLE IF EXISTS "public"."devices";
CREATE TABLE "public"."devices" (
  "id" text COLLATE "pg_catalog"."default" NOT NULL,
  "name" text COLLATE "pg_catalog"."default" NOT NULL,
  "serial_number" text COLLATE "pg_catalog"."default" NOT NULL,
  "trial_end_date" timestamp(6),
  "created_at" timestamp(6) DEFAULT CURRENT_TIMESTAMP
)
;

-- ----------------------------
-- Uniques structure for table devices
-- ----------------------------
ALTER TABLE "public"."devices" ADD CONSTRAINT "devices_serial_number_key" UNIQUE ("serial_number");

-- ----------------------------
-- Primary Key structure for table devices
-- ----------------------------
ALTER TABLE "public"."devices" ADD CONSTRAINT "devices_pkey" PRIMARY KEY ("id");
