/*
  Warnings:

  - You are about to drop the `Player` table. If the table is not empty, all the data it contains will be lost.
  - Added the required column `firstName` to the `Game` table without a default value. This is not possible if the table is not empty.
  - Added the required column `firstProvisional` to the `Game` table without a default value. This is not possible if the table is not empty.
  - Added the required column `firstRating` to the `Game` table without a default value. This is not possible if the table is not empty.
  - Added the required column `secondName` to the `Game` table without a default value. This is not possible if the table is not empty.
  - Added the required column `secondProvisional` to the `Game` table without a default value. This is not possible if the table is not empty.
  - Added the required column `secondRating` to the `Game` table without a default value. This is not possible if the table is not empty.
  - Made the column `firstId` on table `Game` required. This step will fail if there are existing NULL values in that column.
  - Made the column `secondId` on table `Game` required. This step will fail if there are existing NULL values in that column.

*/
-- DropForeignKey
ALTER TABLE "Game" DROP CONSTRAINT "Game_firstId_fkey";

-- DropForeignKey
ALTER TABLE "Game" DROP CONSTRAINT "Game_secondId_fkey";

-- DropIndex
DROP INDEX "Game_firstId_key";

-- DropIndex
DROP INDEX "Game_secondId_key";

-- AlterTable
ALTER TABLE "Game" ADD COLUMN     "firstName" TEXT NOT NULL,
ADD COLUMN     "firstProvisional" BOOLEAN NOT NULL,
ADD COLUMN     "firstRating" INTEGER NOT NULL,
ADD COLUMN     "secondName" TEXT NOT NULL,
ADD COLUMN     "secondProvisional" BOOLEAN NOT NULL,
ADD COLUMN     "secondRating" INTEGER NOT NULL,
ALTER COLUMN "clockInitial" DROP NOT NULL,
ALTER COLUMN "clockIncrement" DROP NOT NULL,
ALTER COLUMN "firstId" SET NOT NULL,
ALTER COLUMN "secondId" SET NOT NULL;

-- DropTable
DROP TABLE "Player";
