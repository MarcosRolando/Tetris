//
// Created by marcosrolando on 4/01/21.
//

#include "TextureRepository.h"

#define PLAYER_GHOST_PATH "/var/Argentum/Assets/Images/Miscellaneous/PlayerGhost.png"

/*
 * PRIVATE
 */

/*
 * PUBLIC
 */

void textureRepository_init(TextureRepository_t* this, SDL_Renderer* renderer) {

}

const Texture_t* textureRepository_get_texture(const TextureRepository_t* this, TextureID texture) {

}

void textureRepository_release(TextureRepository_t* this) {

}




TextureRepository::TextureRepository(SDL_Renderer& renderer) : renderer(renderer) {
    _loadClothing();
    _loadHeads();
    _loadWeapons();
    _loadTiles();
    _loadStructures();
    _loadNPCS();
    _loadDrops();
    _loadMiscellaneous();
    _loadUI();
}

void TextureRepository::_loadUI() {
    _setImage(Background, BACKGROUND_PATH, 1495, 937, 0
            , 0, 1, {-1, -1, -1});
    _setImage(MainMenu, MAIN_MENU_PATH, 1499, 937, 0,
            0, 1, {-1, -1, -1});
}

void TextureRepository::_loadMiscellaneous() {
    _setSpellImage(Explosion, EXPLOSION_PATH, 256, 256, -10, -10);
    _setSpellImage(MagicArrow, MAGIC_ARROW_PATH, 96, 100, 20, 15);
    _setSpellImage(MagicMissile, MAGIC_MISSILE_PATH, 128, 128, 8, 5);
    _setSpellImage(Heal, HEAL_PATH, 76, 76, 25, 20);
    _setImage(SimpleArrow, SIMPLE_ARROW_PATH, 32, 32, 45, 45, 1);
    _setImage(CompositeArrow, COMPOSITE_ARROW_PATH, 32, 32, 45, 45, 1);
}

void TextureRepository::_loadDrops() {
    _setImage(BlueTunicDrop, BLUE_TUNIC_DROP_PATH, 32, 32, 30, 30, 2);
    _setImage(LongSwordDrop, LONG_SWORD_DROP_PATH, 32, 32, 33, 30, 2);
    _setImage(LinkedStaffDrop, LINKED_STAFF_DROP_PATH, 32, 32, 30, 30, 2);
    _setImage(GnarledStaffDrop, GNARLED_STAFF_DROP_PATH, 32, 32, 35, 30, 2);
    _setImage(MagicHatDrop, MAGIC_HAT_DROP_PATH, 32, 32, 50, 45);
    _setImage(HealthPotion, HEALTH_POTION_PATH, 32, 32, 50, 45);
    _setImage(ManaPotion, MANA_POTION_PATH, 32, 32, 50, 45);
    _setImage(CommonClothingDrop, COMMON_CLOTHING_DROP_PATH, 32, 32, 35, 30, 2);
    _setImage(KingArmorDrop, KING_ARMOR_DROP_PATH, 32, 32, 35, 30, 2);
    _setImage(LeatherArmorDrop, LEATHER_ARMOR_DROP_PATH, 32, 32, 35, 30, 2);
    _setImage(PlateArmorDrop, PLATE_ARMOR_DROP_PATH, 16, 32, 48, 35, 2);
    _setImage(HoodDrop, HOOD_DROP_PATH, 32, 32, 50, 45);
    _setImage(IronHelmetDrop, IRON_HELMET_DROP_PATH, 32, 32, 50, 45);
    _setImage(IronShieldDrop, IRON_SHIELD_DROP_PATH, 32, 32, 35, 30, 2);
    _setImage(TurtleShieldDrop, TURTLE_SHIELD_DROP_PATH, 32, 32, 50, 45);
    _setImage(AshRodDrop, ASH_ROD_DROP_PATH, 32, 32, 35, 30, 2);
    _setImage(AxeDrop, AXE_DROP_PATH, 32, 32, 32, 30, 2);
    _setImage(CompositeBowDrop, COMPOSITE_BOW_DROP_PATH, 32, 32, 32, 30, 2);
    _setImage(ElvenFluteDrop, ELVEN_FLUTE_DROP_PATH, 32, 32, 32, 30, 2);
    _setImage(SimpleBowDrop, SIMPLE_BOW_DROP_PATH, 32, 32, 32, 30, 2);
    _setImage(WarHammerDrop, WAR_HAMMER_DROP_PATH, 32, 32, 32, 28, 2);
    _setImage(Gold, GOLD_PATH, 32, 32, 45, 50, 1);
}

void TextureRepository::_loadClothing() {
    _setBodyImage(BlueTunic, BLUE_TUNIC_PATH);
    _setBodyImage(CommonClothing, COMMON_CLOTHING_PATH);
    _setShieldImage(IronShield, IRON_SHIELD_PATH);
    _setBodyImage(LeatherArmor, LEATHER_ARMOR_PATH);
    _setBodyImage(PlateArmor, PLATE_ARMOR_PATH);
    _setBodyImage(KingArmor, KING_ARMOR_PATH);
    _setShieldImage(TurtleShield, TURTLE_SHIELD_PATH);
    _setHelmetImage(Hood, HOOD_PATH);
    _setHelmetImage(IronHelmet, IRON_HELMET_PATH);
    _setHelmetImage(MagicHat, MAGIC_HAT_PATH, -1, -24);
}

void TextureRepository::_loadHeads() {
    _setHeadImage(DwarfHead, DWARF_HEAD_PATH);
    _setHeadImage(ElfHead, ELF_HEAD_PATH);
    _setHeadImage(GnomeHead, GNOME_HEAD_PATH);
    _setHeadImage(HumanHead, HUMAN_HEAD_PATH);
}

void TextureRepository::_loadWeapons() {
    _setWeaponImage(AshRod, ASH_ROD_PATH);
    _setWeaponImage(Axe, AXE_PATH);
    _setWeaponImage(CompositeBow, COMPOSITE_BOW_PATH);
    _setWeaponImage(LinkedStaff, LINKED_STAFF_PATH);
    _setWeaponImage(GnarledStaff, GNARLED_STAFF_PATH);
    _setWeaponImage(LongSword, LONG_SWORD_PATH);
    _setWeaponImage(SimpleBow, SIMPLE_BOW_PATH);
    _setWeaponImage(WarHammer, WAR_HAMMER_PATH);
}

void TextureRepository::_loadTiles() {
    _setTileImage(Grass, GRASS_PATH, false);
    _setTileImage(PrettyGrass, PRETTY_GRASS_PATH, false);
    _setTileImage(PrettyRoad, PRETTY_ROAD_PATH, false);
    _setTileImage(DeadGrass, DEAD_GRASS_PATH, false);
    _setTileImage(Water, WATER_PATH, false);
    _setTileImage(DarkWater, DARK_WATER_PATH, false);
    _setTileImage(Sand, SAND_PATH, true);
}

void TextureRepository::_loadStructures() {
    _setImage(Tree, TREE_PATH, 256, 256, -60, -180);
    _setImage(LongTree, LONG_TREE_PATH, 256, 256, -60, -180);
    _setImage(FatTree, FAT_TREE_PATH, 256, 256, -60, -180);
    _setImage(PalmTree, PALM_TREE_PATH, 256, 256, -60, -180);
    _setImage(DeadTree, DEAD_TREE_PATH, 256, 256, -60, -160);
    _setImage(Bush, BUSH_PATH, 75, 65, 35, 35);
    _setImage(BoneGuy, BONE_GUY_PATH, 75, 65, 30, 40);
    _setImage(BrokenRipStone, BROKEN_RIP_STONE_PATH, 75, 65, 30, 20);
    _setImage(DeadGuy, DEAD_GUY_PATH, 75, 65, 30, -60, 2);
    _setImage(VeryDeadGuy, VERY_DEAD_GUY_PATH, 75, 65, 0, 10, 2);
    _setImage(HangedGuy, HANGED_GUY_PATH, 75, 65, 5, -60, 2);
    _setImage(RipStone, RIP_STONE_PATH, 75, 65, 30, 40);
    _setImage(DeadBush, DEAD_BUSH_PATH, 75, 65, 30, 40);
    _setImage(House1, HOUSE1_PATH, 196, 200, 40, -150);
    _setImage(House2, HOUSE2_PATH, 181, 213, 40, -150);
    _setImage(House3, HOUSE3_PATH, 200, 239, 30, -165);
    _setImage(SunkenShip, SUNKEN_SHIP_PATH, 256, 256, -120, -10, 2);
    _setImage(SunkenColumn, SUNKEN_COLUMN_PATH, 256, 256, 5, -185);
}

void TextureRepository::_loadNPCS() {
    _setNPCImage(Skeleton, SKELETON_PATH, 25, 52, 35, 30);
    _setNPCImage(Goblin, GOBLIN_PATH, 24, 31, 38, 48);
    _setNPCImage(Zombie, ZOMBIE_PATH, 25, 45, 35, 30);
    _setNPCImage(Spider, SPIDER_PATH, 34, 34, 30, 48);
    _setNPCImage(Priest, PRIEST_PATH, 25, 45, 37, 33);
    _setNPCImage(Trader, TRADER_PATH, 24, 48,37, 33);
    _setNPCImage(Banker, BANKER_PATH, 25, 45, 37, 33);
    _setNPCImage(Guard, GUARD_PATH, 28, 52, 37, 33);
    _setNPCImage(PlayerGhost, PLAYER_GHOST_PATH, 47, 71, 20, -10); /*tiene el mismo formato*/
}

void TextureRepository::_setImage(TextureID TextureID, std::string&& image,
                    int width, int height, int xOffset, int yOffset, int scale, ColorKey_t key) {
    try {
        textures.emplace(TextureID, renderer);
        Texture& texture = textures.at(TextureID);
        texture.loadFromFile(image, key, xOffset, yOffset, scale);
        _addSprites(texture, width, height);
    } catch (TPException& e) {
        throw TPException("Failed to load %s sprite sheet texture!\n", image.c_str());
    }
}

void TextureRepository::_setSpellImage(TextureID TextureID, std::string&& spellImage,
                                           int width, int height, int xOffset, int yOffset) {
    try {
        ColorKey_t key = {0, 0, 0};
        textures.emplace(TextureID, renderer);
        Texture& texture = textures.at(TextureID);
        texture.loadFromFile(spellImage, key, xOffset, yOffset);
        _addSpellSprites(texture, 0, width, height);
        _addSpellSprites(texture, height, width, height);
        _addSpellSprites(texture, 2*height, width, height);
        _addSpellSprites(texture, 3*height, width, height);
    } catch (TPException& e) {
        throw TPException("Failed to load %s sprite sheet texture!\n", spellImage.c_str());
    }
}

void TextureRepository::_setTileImage(TextureID TextureID, std::string&& tileImage, bool individualTile) {
    try {
        textures.emplace(TextureID, renderer);
        Texture& texture = textures.at(TextureID);
        texture.loadFromFile(tileImage);
        _addTileSprites(texture, 0, individualTile);
    } catch (TPException& e) {
        throw TPException("Failed to load %s sprite sheet texture!\n", tileImage.c_str());
    }
}

void TextureRepository::_setNPCImage(TextureID TextureID, std::string&& npcImage, int width, int height
                                        , int xOffset, int yOffset) {
    try {
        ColorKey_t key = {0, 0, 0};
        textures.emplace(TextureID, renderer);
        Texture& texture = textures.at(TextureID);
        texture.loadFromFile(npcImage, key, xOffset, yOffset);
        /*Front*/
        _addNPCSprites(texture, 0, false, width, height);
        /*Back*/
        _addNPCSprites(texture, height, false, width, height);
        /*Left*/
        _addNPCSprites(texture, 2*height, true, width, height);
        /*Rigth*/
        _addNPCSprites(texture, 3*height, true, width, height);
    } catch (TPException& e) {
        throw TPException("Failed to load %s sprite sheet texture!\n", npcImage.c_str());
    }
}

void TextureRepository::_addNPCSprites(Texture& texture, int y, bool lateralSide, int width, int height) {
    for (int i = 0; i < 5; ++i) {
        texture.addSprite(width*i, y, width, height);
    }
    if (lateralSide) texture.addSprite(4*width, y, width, height);
    else texture.addSprite(5*width, y, width, height);
}

void TextureRepository::_setBodyImage(TextureID TextureID, std::string&& bodyImage) {
    try {
        ColorKey_t key = {0, 0, 0};
        textures.emplace(TextureID, renderer);
        Texture& texture = textures.at(TextureID);
        texture.loadFromFile(bodyImage, key);
        /*Front*/
        _addBodySprites(texture, 0, false);
        /*Back*/
        _addBodySprites(texture, 45, false);
        /*Left*/
        _addBodySprites(texture, 90, true);
        /*Rigth*/
        _addBodySprites(texture, 135, true);
    } catch (TPException& e) {
        throw TPException("Failed to load %s sprite sheet texture!\n", bodyImage.c_str());
    }
}

void TextureRepository::_setWeaponImage(TextureID TextureID, std::string&& weaponImage) {
    try {
        ColorKey_t key = {0, 0, 0};
        textures.emplace(TextureID, renderer);
        Texture& texture = textures.at(TextureID);
        texture.loadFromFile(weaponImage, key);
        /*Front*/
        _addWeaponSprites(texture, 0, false);
        /*Back*/
        _addWeaponSprites(texture, 45, false);
        /*Left*/
        _addWeaponSprites(texture, 90, true);
        /*Rigth*/
        _addWeaponSprites(texture, 135, true);
    } catch (TPException& e) {
        throw TPException("Failed to load %s sprite sheet texture!\n", weaponImage.c_str());
    }
}

void TextureRepository::_addWeaponSprites(Texture& texture, int y, bool lateralSide) {
    texture.addSprite(0, y, 24, 45);
    texture.addSprite(25, y, 25, 45);
    texture.addSprite(51, y - 1, 23, 45);
    texture.addSprite(76, y - 1, 23, 45);
    texture.addSprite(101, y - 1, 24, 45);
    if (lateralSide) texture.addSprite(101, y, 24, 45);
    else texture.addSprite(126, y, 25, 45);
}

void TextureRepository::_addBodySprites(Texture& texture, int y, bool lateralSide) {
    texture.addSprite(0, y, 24, 45); /*hasta 24 porque sino en la plate armor hay un poco de la otra imagen*/
    texture.addSprite(25, y, 25, 45);
    texture.addSprite(51, y, 24, 45); /*pongo 51 porque sino se veia un poco del pie de otro en algunas ropas*/
    texture.addSprite(75, y, 25, 45);
    texture.addSprite(100, y, 25, 45);
    if (lateralSide) texture.addSprite(100, y, 25, 45);
    else texture.addSprite(125, y, 25, 45);
}

void TextureRepository::_setHeadImage(TextureID TextureID, std::string&& headImage) {
    try {
        ColorKey_t key = {0, 0, 0};
        textures.emplace(TextureID, renderer);
        Texture& texture = textures.at(TextureID);
        texture.loadFromFile(headImage, key);
        texture.addSprite(0, 0, 17, 15);
        texture.addSprite(17, 0, 17, 15);
        texture.addSprite(34, 0, 17, 15);
        texture.addSprite(51, 0, 17, 15);
    } catch (TPException& e) {
        throw TPException("Failed to load %s sprite sheet texture!\n", headImage.c_str());
    }
}

void TextureRepository::_setHelmetImage(TextureID TextureID, std::string&& helmetImage,
                                                int xOffset, int yOffset) {
    try {
        ColorKey_t key = {0, 0, 0};
        textures.emplace(TextureID, renderer);
        Texture& texture = textures.at(TextureID);
        texture.loadFromFile(helmetImage, key, xOffset, yOffset);
        texture.addSprite(0, 0, 17, 17);
        texture.addSprite(17, 0, 17, 17);
        texture.addSprite(34, 0, 17, 17);
        texture.addSprite(51, 0, 17, 17);
    } catch (TPException& e) {
        throw TPException("Failed to load %s sprite sheet texture!\n", helmetImage.c_str());
    }
}

void TextureRepository::_setShieldImage(TextureID TextureID, std::string&& shieldImage) {
    try {
        ColorKey_t key = {0, 0, 0};
        textures.emplace(TextureID, renderer);
        Texture& texture = textures.at(TextureID);
        texture.loadFromFile(shieldImage, key);
        /*Front*/
        _addShieldSprites(texture, 0, false);
        /*Back*/
        _addShieldSprites(texture, 45, false);
        /*Left*/
        _addShieldSprites(texture, 90, true);
        /*Rigth*/
        _addShieldSprites(texture, 135, true);
    } catch (TPException& e) {
        throw TPException("Failed to load %s sprite sheet texture!\n", shieldImage.c_str());
    }
}

void TextureRepository::_addShieldSprites(Texture& texture, int y, bool lateralSide) {
    texture.addSprite(0, y, 25, 35);
    texture.addSprite(26, y, 25, 35);
    texture.addSprite(51, y, 24, 35);
    texture.addSprite(76, y, 25, 35);
    texture.addSprite(101, y, 24, 35);
    if (lateralSide) texture.addSprite(101, y, 24, 35);
    else texture.addSprite(126, y, 25, 35);
}

void TextureRepository::_addTileSprites(Texture& texture, int y, bool individualTile) {
    texture.addSprite(0, 0, TILE_WIDTH, TILE_HEIGHT);
    if (!individualTile) {
        texture.addSprite(TILE_WIDTH, 0, TILE_WIDTH, TILE_HEIGHT);
        texture.addSprite(2*TILE_WIDTH, 0, TILE_WIDTH, TILE_HEIGHT);
        texture.addSprite(3*TILE_WIDTH, 0, TILE_WIDTH, TILE_HEIGHT);
    }
}

void TextureRepository::_addSprites(Texture& texture, int width, int height) {
    texture.addSprite(0, 0, width, height);
}

void TextureRepository::_addSpellSprites(Texture& texture, int y, int width, int height) {
    for (int i = 0; i < 6; ++i) {
        texture.addSprite(width*i, y, width, height);
    }
}

Texture& TextureRepository::getTexture(TextureID texture) {
    return textures.at(texture);
}

SDL_Renderer &TextureRepository::getRenderer() const {
    return renderer;
}
