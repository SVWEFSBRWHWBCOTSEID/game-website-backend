-- CreateTable
CREATE TABLE "Game" (
    "id" TEXT NOT NULL,
    "createdAt" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "rated" BOOLEAN NOT NULL,
    "gameKey" TEXT NOT NULL,
    "clockInitial" INTEGER,
    "clockIncrement" INTEGER,
    "firstUsername" TEXT,
    "secondUsername" TEXT,
    "firstRating" INTEGER,
    "secondRating" INTEGER,
    "firstProv" BOOLEAN,
    "secondProv" BOOLEAN,
    "ratingMin" INTEGER NOT NULL,
    "ratingMax" INTEGER NOT NULL,
    "startPos" TEXT,
    "moves" TEXT NOT NULL,
    "firstTime" INTEGER,
    "secondTime" INTEGER,
    "lastMoveTime" BIGINT NOT NULL,
    "status" TEXT NOT NULL,
    "winType" TEXT,
    "drawOffer" TEXT NOT NULL,
    "rematchOffer" TEXT NOT NULL,
    "randomSide" BOOLEAN NOT NULL,

    CONSTRAINT "Game_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "User" (
    "username" TEXT NOT NULL,
    "createdAt" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "password" TEXT NOT NULL,
    "guest" BOOLEAN NOT NULL,
    "country" TEXT NOT NULL,
    "location" TEXT NOT NULL,
    "bio" TEXT NOT NULL,
    "firstName" TEXT NOT NULL,
    "lastName" TEXT NOT NULL,
    "url" TEXT NOT NULL,
    "playing" TEXT,
    "canStartGame" BOOLEAN NOT NULL
);

-- CreateTable
CREATE TABLE "Preferences" (
    "username" TEXT NOT NULL,
    "showTenthSeconds" TEXT NOT NULL,
    "showProgressBars" BOOLEAN NOT NULL,
    "playCriticalSound" BOOLEAN NOT NULL,
    "confirmResign" BOOLEAN NOT NULL,
    "boardScroll" BOOLEAN NOT NULL
);

-- CreateTable
CREATE TABLE "Friend" (
    "type" TEXT NOT NULL,
    "username" TEXT NOT NULL,
    "friendName" TEXT NOT NULL
);

-- CreateTable
CREATE TABLE "Perf" (
    "username" TEXT NOT NULL,
    "gameKey" TEXT NOT NULL,
    "rating" DOUBLE PRECISION NOT NULL,
    "rd" DOUBLE PRECISION NOT NULL,
    "volatility" DOUBLE PRECISION NOT NULL,
    "tau" DOUBLE PRECISION NOT NULL,
    "prog" TEXT NOT NULL,
    "prov" BOOLEAN NOT NULL
);

-- CreateTable
CREATE TABLE "Message" (
    "id" TEXT NOT NULL,
    "gameId" TEXT NOT NULL,
    "username" TEXT NOT NULL,
    "text" TEXT NOT NULL,
    "visibility" TEXT NOT NULL,
    "gameEvent" BOOLEAN NOT NULL,

    CONSTRAINT "Message_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "Conversation" (
    "id" TEXT NOT NULL,
    "username" TEXT NOT NULL,
    "otherName" TEXT NOT NULL,

    CONSTRAINT "Conversation_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "UserMessage" (
    "id" TEXT NOT NULL,
    "createdAt" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "convId" TEXT NOT NULL,
    "username" TEXT NOT NULL,
    "text" TEXT NOT NULL,

    CONSTRAINT "UserMessage_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "Challenge" (
    "id" TEXT NOT NULL,
    "createdAt" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "username" TEXT NOT NULL,
    "opponentName" TEXT NOT NULL,
    "gameId" TEXT NOT NULL,

    CONSTRAINT "Challenge_pkey" PRIMARY KEY ("id")
);

-- CreateIndex
CREATE UNIQUE INDEX "User_username_key" ON "User"("username");

-- CreateIndex
CREATE UNIQUE INDEX "Preferences_username_key" ON "Preferences"("username");

-- CreateIndex
CREATE UNIQUE INDEX "Friend_username_friendName_key" ON "Friend"("username", "friendName");

-- CreateIndex
CREATE UNIQUE INDEX "Perf_username_gameKey_key" ON "Perf"("username", "gameKey");

-- CreateIndex
CREATE UNIQUE INDEX "Conversation_username_otherName_key" ON "Conversation"("username", "otherName");

-- CreateIndex
CREATE UNIQUE INDEX "Challenge_username_key" ON "Challenge"("username");

-- CreateIndex
CREATE UNIQUE INDEX "Challenge_gameId_key" ON "Challenge"("gameId");

-- CreateIndex
CREATE UNIQUE INDEX "Challenge_username_opponentName_key" ON "Challenge"("username", "opponentName");

-- AddForeignKey
ALTER TABLE "Game" ADD CONSTRAINT "Game_firstUsername_fkey" FOREIGN KEY ("firstUsername") REFERENCES "User"("username") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Game" ADD CONSTRAINT "Game_secondUsername_fkey" FOREIGN KEY ("secondUsername") REFERENCES "User"("username") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Preferences" ADD CONSTRAINT "Preferences_username_fkey" FOREIGN KEY ("username") REFERENCES "User"("username") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Friend" ADD CONSTRAINT "Friend_username_fkey" FOREIGN KEY ("username") REFERENCES "User"("username") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Friend" ADD CONSTRAINT "Friend_friendName_fkey" FOREIGN KEY ("friendName") REFERENCES "User"("username") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Perf" ADD CONSTRAINT "Perf_username_fkey" FOREIGN KEY ("username") REFERENCES "User"("username") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Message" ADD CONSTRAINT "Message_gameId_fkey" FOREIGN KEY ("gameId") REFERENCES "Game"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Conversation" ADD CONSTRAINT "Conversation_username_fkey" FOREIGN KEY ("username") REFERENCES "User"("username") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "UserMessage" ADD CONSTRAINT "UserMessage_convId_fkey" FOREIGN KEY ("convId") REFERENCES "Conversation"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Challenge" ADD CONSTRAINT "Challenge_username_fkey" FOREIGN KEY ("username") REFERENCES "User"("username") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Challenge" ADD CONSTRAINT "Challenge_opponentName_fkey" FOREIGN KEY ("opponentName") REFERENCES "User"("username") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Challenge" ADD CONSTRAINT "Challenge_gameId_fkey" FOREIGN KEY ("gameId") REFERENCES "Game"("id") ON DELETE RESTRICT ON UPDATE CASCADE;
