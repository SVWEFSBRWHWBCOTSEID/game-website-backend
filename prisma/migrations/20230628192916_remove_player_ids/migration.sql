/*
  Warnings:

  - You are about to drop the column `firstId` on the `Game` table. All the data in the column will be lost.
  - You are about to drop the column `secondId` on the `Game` table. All the data in the column will be lost.

*/
-- AlterTable
ALTER TABLE "Game" DROP COLUMN "firstId",
DROP COLUMN "secondId";
