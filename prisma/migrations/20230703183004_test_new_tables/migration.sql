/*
  Warnings:

  - You are about to drop the column `firstName` on the `Game` table. All the data in the column will be lost.
  - You are about to drop the column `firstProvisional` on the `Game` table. All the data in the column will be lost.
  - You are about to drop the column `firstRating` on the `Game` table. All the data in the column will be lost.
  - You are about to drop the column `firstTime` on the `Game` table. All the data in the column will be lost.
  - You are about to drop the column `gameName` on the `Game` table. All the data in the column will be lost.
  - You are about to drop the column `moves` on the `Game` table. All the data in the column will be lost.
  - You are about to drop the column `secondName` on the `Game` table. All the data in the column will be lost.
  - You are about to drop the column `secondProvisional` on the `Game` table. All the data in the column will be lost.
  - You are about to drop the column `secondRating` on the `Game` table. All the data in the column will be lost.
  - You are about to drop the column `secondTime` on the `Game` table. All the data in the column will be lost.
  - You are about to drop the column `status` on the `Game` table. All the data in the column will be lost.
  - You are about to drop the column `bio` on the `User` table. All the data in the column will be lost.
  - You are about to drop the column `country` on the `User` table. All the data in the column will be lost.
  - You are about to drop the column `firstName` on the `User` table. All the data in the column will be lost.
  - You are about to drop the column `lastName` on the `User` table. All the data in the column will be lost.
  - You are about to drop the column `location` on the `User` table. All the data in the column will be lost.
  - You are about to drop the column `name` on the `User` table. All the data in the column will be lost.
  - You are about to drop the column `perfs` on the `User` table. All the data in the column will be lost.
  - A unique constraint covering the columns `[username]` on the table `User` will be added. If there are existing duplicate values, this will fail.
  - Changed the type of `gameKey` on the `Game` table. No cast exists, the column would be dropped and recreated, which cannot be done if there is data, since the column is required.
  - Added the required column `username` to the `User` table without a default value. This is not possible if the table is not empty.

*/
-- CreateEnum
CREATE TYPE "GameKey" AS ENUM ('TTT', 'UTTT', 'C4', 'PC');

-- DropIndex
DROP INDEX "User_name_key";

-- AlterTable
ALTER TABLE "Game" DROP COLUMN "firstName",
DROP COLUMN "firstProvisional",
DROP COLUMN "firstRating",
DROP COLUMN "firstTime",
DROP COLUMN "gameName",
DROP COLUMN "moves",
DROP COLUMN "secondName",
DROP COLUMN "secondProvisional",
DROP COLUMN "secondRating",
DROP COLUMN "secondTime",
DROP COLUMN "status",
ADD COLUMN     "firstUsername" TEXT,
ADD COLUMN     "secondUsername" TEXT,
DROP COLUMN "gameKey",
ADD COLUMN     "gameKey" "GameKey" NOT NULL;

-- AlterTable
ALTER TABLE "User" DROP COLUMN "bio",
DROP COLUMN "country",
DROP COLUMN "firstName",
DROP COLUMN "lastName",
DROP COLUMN "location",
DROP COLUMN "name",
DROP COLUMN "perfs",
ADD COLUMN     "username" TEXT NOT NULL;

-- CreateTable
CREATE TABLE "GamePerf" (
    "username" TEXT NOT NULL,
    "gameKey" "GameKey" NOT NULL,
    "games" INTEGER NOT NULL,
    "rating" INTEGER NOT NULL,
    "rd" DOUBLE PRECISION NOT NULL,
    "prog" INTEGER NOT NULL,
    "prov" BOOLEAN NOT NULL
);

-- CreateTable
CREATE TABLE "Profile" (
    "username" TEXT NOT NULL,
    "country" "Country" NOT NULL,
    "location" TEXT NOT NULL,
    "bio" TEXT NOT NULL,
    "firstName" TEXT NOT NULL,
    "lastName" TEXT NOT NULL
);

-- CreateTable
CREATE TABLE "GameState" (
    "gameId" TEXT NOT NULL,
    "moves" TEXT NOT NULL,
    "firstTime" INTEGER,
    "secondTime" INTEGER,
    "status" "GameStatus" NOT NULL
);

-- CreateIndex
CREATE UNIQUE INDEX "GamePerf_username_gameKey_key" ON "GamePerf"("username", "gameKey");

-- CreateIndex
CREATE UNIQUE INDEX "Profile_username_key" ON "Profile"("username");

-- CreateIndex
CREATE UNIQUE INDEX "GameState_gameId_key" ON "GameState"("gameId");

-- CreateIndex
CREATE UNIQUE INDEX "User_username_key" ON "User"("username");

-- AddForeignKey
ALTER TABLE "GamePerf" ADD CONSTRAINT "GamePerf_username_fkey" FOREIGN KEY ("username") REFERENCES "User"("username") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Profile" ADD CONSTRAINT "Profile_username_fkey" FOREIGN KEY ("username") REFERENCES "User"("username") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "GameState" ADD CONSTRAINT "GameState_gameId_fkey" FOREIGN KEY ("gameId") REFERENCES "Game"("id") ON DELETE RESTRICT ON UPDATE CASCADE;
