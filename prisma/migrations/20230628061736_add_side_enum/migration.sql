-- CreateEnum
CREATE TYPE "Side" AS ENUM ('FIRST', 'SECOND', 'RANDOM');

-- DropForeignKey
ALTER TABLE "Game" DROP CONSTRAINT "Game_firstId_fkey";

-- DropForeignKey
ALTER TABLE "Game" DROP CONSTRAINT "Game_secondId_fkey";

-- AlterTable
ALTER TABLE "Game" ALTER COLUMN "firstId" DROP NOT NULL,
ALTER COLUMN "secondId" DROP NOT NULL;

-- AddForeignKey
ALTER TABLE "Game" ADD CONSTRAINT "Game_firstId_fkey" FOREIGN KEY ("firstId") REFERENCES "Player"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Game" ADD CONSTRAINT "Game_secondId_fkey" FOREIGN KEY ("secondId") REFERENCES "Player"("id") ON DELETE SET NULL ON UPDATE CASCADE;
