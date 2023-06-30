/*
  Warnings:

  - You are about to drop the column `c4Provisional` on the `User` table. All the data in the column will be lost.
  - You are about to drop the column `c4Rating` on the `User` table. All the data in the column will be lost.
  - You are about to drop the column `pcProvisional` on the `User` table. All the data in the column will be lost.
  - You are about to drop the column `pcRating` on the `User` table. All the data in the column will be lost.
  - You are about to drop the column `tttProvisional` on the `User` table. All the data in the column will be lost.
  - You are about to drop the column `tttRating` on the `User` table. All the data in the column will be lost.
  - You are about to drop the column `utttProvisional` on the `User` table. All the data in the column will be lost.
  - You are about to drop the column `utttRating` on the `User` table. All the data in the column will be lost.
  - Added the required column `password` to the `User` table without a default value. This is not possible if the table is not empty.
  - Added the required column `perfs` to the `User` table without a default value. This is not possible if the table is not empty.
  - Added the required column `url` to the `User` table without a default value. This is not possible if the table is not empty.

*/
-- CreateEnum
CREATE TYPE "Country" AS ENUM ('US', 'MN', 'UK');

-- AlterTable
ALTER TABLE "User" DROP COLUMN "c4Provisional",
DROP COLUMN "c4Rating",
DROP COLUMN "pcProvisional",
DROP COLUMN "pcRating",
DROP COLUMN "tttProvisional",
DROP COLUMN "tttRating",
DROP COLUMN "utttProvisional",
DROP COLUMN "utttRating",
ADD COLUMN     "bio" TEXT,
ADD COLUMN     "country" "Country",
ADD COLUMN     "firstName" TEXT,
ADD COLUMN     "lastName" TEXT,
ADD COLUMN     "location" TEXT,
ADD COLUMN     "password" TEXT NOT NULL,
ADD COLUMN     "perfs" TEXT NOT NULL,
ADD COLUMN     "playing" TEXT,
ADD COLUMN     "url" TEXT NOT NULL;
