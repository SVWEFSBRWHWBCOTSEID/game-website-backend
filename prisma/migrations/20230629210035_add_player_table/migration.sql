-- CreateTable
CREATE TABLE "User" (
    "name" TEXT NOT NULL,
    "tttRating" INTEGER NOT NULL,
    "tttProvisional" BOOLEAN NOT NULL,
    "utttRating" INTEGER NOT NULL,
    "utttProvisional" BOOLEAN NOT NULL,
    "c4Rating" INTEGER NOT NULL,
    "c4Provisional" BOOLEAN NOT NULL,
    "pcRating" INTEGER NOT NULL,
    "pcProvisional" BOOLEAN NOT NULL
);

-- CreateIndex
CREATE UNIQUE INDEX "User_name_key" ON "User"("name");
