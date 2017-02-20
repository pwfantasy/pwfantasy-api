/*
Navicat MySQL Data Transfer

Source Server         : localhost
Source Server Version : 50717
Source Host           : localhost:3306
Source Database       : pwfantasy

Target Server Type    : MYSQL
Target Server Version : 50717
File Encoding         : 65001

Date: 2017-02-20 00:13:38
*/

SET FOREIGN_KEY_CHECKS=0;

-- ----------------------------
-- Table structure for `championships`
-- ----------------------------
DROP TABLE IF EXISTS `championships`;
CREATE TABLE `championships` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `name` varchar(255) NOT NULL,
  `promotion` int(11) NOT NULL,
  `image` varchar(255) DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

-- ----------------------------
-- Records of championships
-- ----------------------------

-- ----------------------------
-- Table structure for `factions`
-- ----------------------------
DROP TABLE IF EXISTS `factions`;
CREATE TABLE `factions` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `name` varchar(255) NOT NULL,
  `slug` varchar(45) NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

-- ----------------------------
-- Records of factions
-- ----------------------------

-- ----------------------------
-- Table structure for `matches`
-- ----------------------------
DROP TABLE IF EXISTS `matches`;
CREATE TABLE `matches` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `date` datetime NOT NULL,
  `show` int(11) NOT NULL,
  `type` int(11) NOT NULL,
  `decision` int(11) NOT NULL,
  `championship` int(11) DEFAULT NULL COMMENT 'Championship Id that was put on the line for this match',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

-- ----------------------------
-- Records of matches
-- ----------------------------

-- ----------------------------
-- Table structure for `matches_participants`
-- ----------------------------
DROP TABLE IF EXISTS `matches_participants`;
CREATE TABLE `matches_participants` (
  `mid` int(11) NOT NULL COMMENT 'Match Id',
  `tid` int(11) NOT NULL COMMENT 'Talent Id',
  `won` tinyint(1) DEFAULT '0',
  `loss` tinyint(1) DEFAULT '0',
  `disqualified` tinyint(1) DEFAULT '0' COMMENT 'Whether this talent was "officially" disqualified from the match.',
  `champion` tinyint(1) DEFAULT '0' COMMENT '(bool) Whether this talent left as champion. Use this and matches.championship to build championship history',
  `referee` tinyint(1) DEFAULT '0' COMMENT 'Whether this talent officated the match',
  `manager` int(11) DEFAULT NULL COMMENT 'Talent Id for who managed this person (ie: Paul Heyman''s tid). ',
  `interfered` tinyint(1) DEFAULT '0' COMMENT 'Whether this talent "officially" interferred in the match. False if the ref didn''t see it',
  `entrant` int(11) DEFAULT '0' COMMENT 'Entry number. Specifically used for Royal Rumble or Elimination Chamber matches'
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

-- ----------------------------
-- Records of matches_participants
-- ----------------------------

-- ----------------------------
-- Table structure for `match_decisions`
-- ----------------------------
DROP TABLE IF EXISTS `match_decisions`;
CREATE TABLE `match_decisions` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `name` varchar(255) NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

-- ----------------------------
-- Records of match_decisions
-- ----------------------------

-- ----------------------------
-- Table structure for `match_types`
-- ----------------------------
DROP TABLE IF EXISTS `match_types`;
CREATE TABLE `match_types` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `name` varchar(255) NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

-- ----------------------------
-- Records of match_types
-- ----------------------------

-- ----------------------------
-- Table structure for `promotions`
-- ----------------------------
DROP TABLE IF EXISTS `promotions`;
CREATE TABLE `promotions` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `name` varchar(255) NOT NULL,
  `image` varchar(255) DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=3 DEFAULT CHARSET=utf8;

-- ----------------------------
-- Records of promotions
-- ----------------------------
INSERT INTO `promotions` VALUES ('1', 'WWE', null);
INSERT INTO `promotions` VALUES ('2', 'TNA', null);

-- ----------------------------
-- Table structure for `seasons`
-- ----------------------------
DROP TABLE IF EXISTS `seasons`;
CREATE TABLE `seasons` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `start` datetime NOT NULL,
  `end` datetime NOT NULL,
  `promotion` int(11) NOT NULL COMMENT 'Promotion Id this season is associated with',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

-- ----------------------------
-- Records of seasons
-- ----------------------------

-- ----------------------------
-- Table structure for `shows`
-- ----------------------------
DROP TABLE IF EXISTS `shows`;
CREATE TABLE `shows` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `name` varchar(255) NOT NULL,
  `promotion` int(11) NOT NULL,
  `image` varchar(255) DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=6 DEFAULT CHARSET=utf8;

-- ----------------------------
-- Records of shows
-- ----------------------------
INSERT INTO `shows` VALUES ('1', 'Raw', '1', null);
INSERT INTO `shows` VALUES ('2', 'Smackdown LIVE', '1', null);
INSERT INTO `shows` VALUES ('3', 'NXT', '1', null);
INSERT INTO `shows` VALUES ('4', '205 LIVE', '1', null);
INSERT INTO `shows` VALUES ('5', 'Impact Wrestling', '2', null);

-- ----------------------------
-- Table structure for `talent`
-- ----------------------------
DROP TABLE IF EXISTS `talent`;
CREATE TABLE `talent` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `name` varchar(255) NOT NULL,
  `slug` varchar(45) NOT NULL,
  `tier` int(11) NOT NULL,
  `active` tinyint(1) DEFAULT '1',
  `faction` int(11) DEFAULT NULL,
  `championship` int(11) DEFAULT NULL,
  `show` int(11) DEFAULT NULL,
  `image` varchar(255) DEFAULT NULL,
  `bio` text,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=2 DEFAULT CHARSET=utf8;

-- ----------------------------
-- Records of talent
-- ----------------------------
INSERT INTO `talent` VALUES ('1', 'The Rock', 'therock', '1', '1', null, null, null, null, null);

-- ----------------------------
-- Table structure for `users`
-- ----------------------------
DROP TABLE IF EXISTS `users`;
CREATE TABLE `users` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `username` varchar(16) NOT NULL,
  `email` varchar(255) NOT NULL,
  `password` varchar(32) NOT NULL,
  `create_time` datetime DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8 PACK_KEYS=1;

-- ----------------------------
-- Records of users
-- ----------------------------
