-- AlterTable
ALTER TABLE "Game" ALTER COLUMN "firstId" DROP NOT NULL,
ALTER COLUMN "secondId" DROP NOT NULL,
ALTER COLUMN "firstName" DROP NOT NULL,
ALTER COLUMN "firstProvisional" DROP NOT NULL,
ALTER COLUMN "firstRating" DROP NOT NULL,
ALTER COLUMN "secondName" DROP NOT NULL,
ALTER COLUMN "secondProvisional" DROP NOT NULL,
ALTER COLUMN "secondRating" DROP NOT NULL;
