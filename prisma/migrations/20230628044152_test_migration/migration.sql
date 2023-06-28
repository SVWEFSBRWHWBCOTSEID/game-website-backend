-- CreateEnum
CREATE TYPE "GameStatus" AS ENUM ('WAITING', 'STARTED');

-- CreateTable
CREATE TABLE "Game" (
    "id" TEXT NOT NULL,
    "createdAt" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "rated" BOOLEAN NOT NULL,
    "gameKey" TEXT NOT NULL,
    "gameName" TEXT NOT NULL,
    "clockInitial" INTEGER NOT NULL,
    "clockIncrement" INTEGER NOT NULL,
    "firstId" TEXT NOT NULL,
    "secondId" TEXT NOT NULL,
    "startPos" TEXT NOT NULL,
    "moves" TEXT[],
    "firstTime" INTEGER NOT NULL,
    "secondTime" INTEGER NOT NULL,
    "status" "GameStatus" NOT NULL,

    CONSTRAINT "Game_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "Player" (
    "id" TEXT NOT NULL,
    "name" TEXT NOT NULL,
    "provisional" BOOLEAN NOT NULL,
    "rating" INTEGER NOT NULL,

    CONSTRAINT "Player_pkey" PRIMARY KEY ("id")
);

-- CreateIndex
CREATE UNIQUE INDEX "Game_firstId_key" ON "Game"("firstId");

-- CreateIndex
CREATE UNIQUE INDEX "Game_secondId_key" ON "Game"("secondId");

-- AddForeignKey
ALTER TABLE "Game" ADD CONSTRAINT "Game_firstId_fkey" FOREIGN KEY ("firstId") REFERENCES "Player"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Game" ADD CONSTRAINT "Game_secondId_fkey" FOREIGN KEY ("secondId") REFERENCES "Player"("id") ON DELETE RESTRICT ON UPDATE CASCADE;
