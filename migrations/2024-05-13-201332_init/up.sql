-- Your SQL goes here

CREATE TABLE `user` (
  `id` int NOT NULL AUTO_INCREMENT,
  `email` varchar(191) COLLATE utf8mb4_unicode_ci NOT NULL,
  `password` varchar(64) COLLATE utf8mb4_unicode_ci NOT NULL,
  `role` varchar(191) COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT 'Operator',
  `position` varchar(191) COLLATE utf8mb4_unicode_ci DEFAULT NULL,
  `first_name` varchar(191) COLLATE utf8mb4_unicode_ci DEFAULT NULL,
  `last_name` varchar(191) COLLATE utf8mb4_unicode_ci DEFAULT NULL,
  `bio` varchar(191) COLLATE utf8mb4_unicode_ci DEFAULT NULL,
  `image` varchar(191) COLLATE utf8mb4_unicode_ci DEFAULT NULL,
  `created_at` datetime(3) NOT NULL DEFAULT CURRENT_TIMESTAMP(3),
  `updated_at` datetime(3) DEFAULT NULL,

  PRIMARY KEY (`id`),
  UNIQUE KEY `User_email_key` (`email`)
) ENGINE=InnoDB AUTO_INCREMENT=9 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE `product` (
  `id` int NOT NULL AUTO_INCREMENT,
  `name` varchar(191) COLLATE utf8mb4_unicode_ci NOT NULL,
  `description` varchar(191) COLLATE utf8mb4_unicode_ci NOT NULL,

  PRIMARY KEY (`id`),
  UNIQUE KEY `Product_name_key` (`name`)
) ENGINE=InnoDB AUTO_INCREMENT=1412 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE `productlot` (
  `id` int NOT NULL AUTO_INCREMENT,
  `lot_number` varchar(191) COLLATE utf8mb4_unicode_ci NOT NULL,
  `internal_reference` varchar(191) COLLATE utf8mb4_unicode_ci NOT NULL,
  `product_name` varchar(191) COLLATE utf8mb4_unicode_ci NOT NULL,
  `quantity` int NOT NULL DEFAULT '0',

  PRIMARY KEY (`id`),
  UNIQUE KEY `ProductLot_lot_number_key` (`lot_number`),
  UNIQUE KEY `ProductLot_internal_reference_key` (`internal_reference`),
  KEY `ProductLot_product_name_fkey` (`product_name`),
  CONSTRAINT `ProductLot_product_name_fkey` FOREIGN KEY (`product_name`) REFERENCES `product` (`name`) ON DELETE RESTRICT ON UPDATE CASCADE
) ENGINE=InnoDB AUTO_INCREMENT=16 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE `warehouse` (
  `id` int NOT NULL AUTO_INCREMENT,
  `name` varchar(191) COLLATE utf8mb4_unicode_ci NOT NULL,

  PRIMARY KEY (`id`),
  UNIQUE KEY `Warehouse_name_key` (`name`)
) ENGINE=InnoDB AUTO_INCREMENT=6 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE `inventorybay` (
  `id` int NOT NULL AUTO_INCREMENT,
  `name` varchar(191) COLLATE utf8mb4_unicode_ci NOT NULL,
  `warehouse_name` varchar(191) COLLATE utf8mb4_unicode_ci NOT NULL,
  `max_unique_lots` int NOT NULL DEFAULT '1',

  PRIMARY KEY (`id`),
  UNIQUE KEY `InventoryBay_name_key` (`name`),
  KEY `InventoryBay_warehouse_name_fkey` (`warehouse_name`),
  CONSTRAINT `InventoryBay_warehouse_name_fkey` FOREIGN KEY (`warehouse_name`) REFERENCES `warehouse` (`name`) ON DELETE RESTRICT ON UPDATE CASCADE
) ENGINE=InnoDB AUTO_INCREMENT=626 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE `inventory` (
  `id` int NOT NULL AUTO_INCREMENT,
  `lot_number` varchar(191) COLLATE utf8mb4_unicode_ci NOT NULL,
  `location` varchar(191) COLLATE utf8mb4_unicode_ci NOT NULL,
  `quantity` int NOT NULL,
  `created_at` datetime(3) NOT NULL DEFAULT CURRENT_TIMESTAMP(3),
  `created_by` varchar(191) COLLATE utf8mb4_unicode_ci NOT NULL,
  `updated_at` datetime(3) DEFAULT NULL,
  `updated_by` varchar(191) COLLATE utf8mb4_unicode_ci DEFAULT NULL,

  PRIMARY KEY (`id`),
  KEY `Inventory_created_by_fkey` (`created_by`),
  KEY `Inventory_lot_number_fkey` (`lot_number`),
  KEY `Inventory_location_fkey` (`location`),
  KEY `Inventory_updated_by_fkey` (`updated_by`),
  CONSTRAINT `Inventory_created_by_fkey` FOREIGN KEY (`created_by`) REFERENCES `user` (`email`) ON DELETE RESTRICT ON UPDATE CASCADE,
  CONSTRAINT `Inventory_location_fkey` FOREIGN KEY (`location`) REFERENCES `inventorybay` (`name`) ON DELETE RESTRICT ON UPDATE CASCADE,
  CONSTRAINT `Inventory_lot_number_fkey` FOREIGN KEY (`lot_number`) REFERENCES `productlot` (`lot_number`) ON DELETE RESTRICT ON UPDATE CASCADE,
  CONSTRAINT `Inventory_updated_by_fkey` FOREIGN KEY (`updated_by`) REFERENCES `user` (`email`) ON DELETE SET NULL ON UPDATE CASCADE
) ENGINE=InnoDB AUTO_INCREMENT=19 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE `log` (
  `id` int NOT NULL AUTO_INCREMENT,
  `from_location` varchar(191) COLLATE utf8mb4_unicode_ci NOT NULL,
  `to_location` varchar(191) COLLATE utf8mb4_unicode_ci NOT NULL,
  `date_time` datetime(3) NOT NULL DEFAULT CURRENT_TIMESTAMP(3),
  `user` varchar(191) COLLATE utf8mb4_unicode_ci NOT NULL,
  `lot_number` varchar(191) COLLATE utf8mb4_unicode_ci NOT NULL,
  `quantity_moved` int NOT NULL,
  `comments` varchar(191) COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '',

  PRIMARY KEY (`id`),
  KEY `Log_from_location_fkey` (`from_location`),
  KEY `Log_to_location_fkey` (`to_location`),
  KEY `Log_user_fkey` (`user`),
  KEY `Log_lot_number_fkey` (`lot_number`),
  CONSTRAINT `Log_from_location_fkey` FOREIGN KEY (`from_location`) REFERENCES `inventorybay` (`name`) ON DELETE RESTRICT ON UPDATE CASCADE,
  CONSTRAINT `Log_lot_number_fkey` FOREIGN KEY (`lot_number`) REFERENCES `productlot` (`lot_number`) ON DELETE RESTRICT ON UPDATE CASCADE,
  CONSTRAINT `Log_to_location_fkey` FOREIGN KEY (`to_location`) REFERENCES `inventorybay` (`name`) ON DELETE RESTRICT ON UPDATE CASCADE,
  CONSTRAINT `Log_user_fkey` FOREIGN KEY (`user`) REFERENCES `user` (`email`) ON DELETE RESTRICT ON UPDATE CASCADE
) ENGINE=InnoDB AUTO_INCREMENT=42 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;




