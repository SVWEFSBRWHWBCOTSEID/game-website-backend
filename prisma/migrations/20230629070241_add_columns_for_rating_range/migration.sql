/*
  Warnings:

  - Added the required column `ratingMax` to the `Game` table without a default value. This is not possible if the table is not empty.
  - Added the required column `ratingMin` to the `Game` table without a default value. This is not possible if the table is not empty.

*/
-- AlterTable
ALTER TABLE "Game" ADD COLUMN     "ratingMax" INTEGER NOT NULL,
ADD COLUMN     "ratingMin" INTEGER NOT NULL;
