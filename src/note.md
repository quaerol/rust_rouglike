
https://bfnightly.bracketproductions.com/chapter_10.html

1 将rust 代码 编译魏WASM  被游览器运行，将js 文件（是不是rust 被编译为js 文件），将rust在窗口中显示的内容显示到浏览器中，bindgen 
rltk 是 rouglike 的 工具包
2，在浏览器运行本地的html  和 js 文件，需要打开本地的web 服务

3，浏览器的兼容问题，谷歌浏览器

４，元组的join　类似database（数据表）的join，

如何创建lib 库 

git config --global core.autocrlf true
git add .
git commit -m ''
git push

## 2.1 实体和组件
## 2.4. Field of View
随着人物的移动逐渐显示地图，将人物周围一定范围内的地图显示出来，其余全部都是黑色，代表呗人物看到的范围
map refactor  ,将与地图相关的数据和函数放在一个，struct map , impl map， 这样可以直接传递map 给使用者，而不是描述地图的一个 vector （一个数据类型）

the filed of view component 视场的组件
不止玩家有可见范围，而且怪物也是有的，所以 因为特性相同，所以可将这个特性抽象为一个组件，Viewshed component （我可以从这个世界看到什么）
在 struct Viewshed 中，有一个 list 代表哪些title 可以被拥有这个组件的entiy 看到，

将新的组件注册到ecs 中，
将Viewshed 组件添加到players中

a new system: generic viewsheds（通用视域）
系统中处理的数据也需要一个struct  来抽象保存
然后为这个系统impl  System 这个 trait
pub struct VisibilitySystem {}
impl<'a> System<'a> for VisibilitySystem 

调用系统，

ask RLTK（一个游戏的组件） for a viewshed: trait implementation 特征的实现

我们自己定义的地图被RLTK 正常使用，需要我们为map 实现一部分 RLTK 提供的trait


map 中有哪些内容，地图中title 的特性，分类，哪些玩家还没探索，不可见的，哪些探索了但是玩家看不到的（灰色），那些是玩家看到的
优化性能，标志位，什么时候渲染，什么时候不渲染，是不是每帧都需要渲染重复的东西，更新每个title的状态，如果玩家没有移动，那么他的可见范围还是这样，不会改变，那么这些可见范围内的title 的状态和标志不需要再改变，

是为了保存状态，信息，

## 2.5 monster
1，rendering a monster in the center ot each room
怪物有renderable 组件，有 viewshed 组件，

我们需要再player 看到monster 的时候，才渲染怪物，
因为monster 也是地图的一部分，所以我们只需要检查（遍历）这个怪物 图块是否可见，可见才渲染，

2，add some monster variety
g oblins（哥布林） 和 o rcs（半兽人）
monster spawner 怪物生成器
随机生成不同的种类的怪物，

3，make the monster think（怪物的AI）
怪物的AI是不是可以使用模型来进行训练，一部分来作为怪物，一部人来作为，

怪物只会在玩家移动是进行思考，
You could let monsters think every time anything moves (and you probably will when you get into deeper simulation), but for now lets quiet them down a bit - and have them react if they can see the player.

如何 修改commit 

在网上发布的一些文章，有时候只是需要有，而不是文笔怎样，怎样声情并茂，而是陈述什么事情，把一些必要的信息列出来，然后加上一些常见的套话，

一些复杂的任务，需要记录下来，形成文本，这样 都有一个基本，都可以一起看，一起讨论，确定具体的做法，然后开始行动，而不是在哪里讨论什么该如何做，


有些人，可以暂时的赚了一部分钱，但是因为一些人生的脚本，这些钱并不会被留在在他的手上，他最后还是会败尽家财


## 2.6 Dealing Damage
教程链接
https://bfnightly.bracketproductions.com/chapter_9.html

Specs 的 教程链接
https://specs.amethyst.rs/docs/tutorials/01_intro

## 2.6 Dealing Damage
1, monster chase player
monster 的行动路径，哪些房间是可以走过的
RLTK 提供了 BaseMap trait  需要我们的 Map 实现 BaseMap

2，怪物不会走在各自身上，也不会走在玩家的身上，而且不会被阻塞在某一个地方
地图的图块一个新的属性，是否被阻塞pub blocked : Vec<bool>
这个图块是否被堵塞，TileType::Wall 是会被阻塞的
如果图块被阻塞blocked，那么不能exit

3，新的组件BlocksTile，注册组件, player and monster both have BlocksTile component

4，填充blocked list，
系统map_indexing_system.rs
将所有有BlocksTile 组件的tile 添加到地图的blocked list
将这个系统注册到 run_system 
怪物只有距离玩家一段距离才会yell(大叫)
防止玩家从怪物身上走过

5，斜线运动
怪物斜线运动，玩家斜线运动

6，战斗状态
CombatStats Component hp defense power 
给玩家添加战斗状态，

7，indexing what is where ,知道图块（tile）上的内容map 的 tile_content 存储tile 上的内容，
map_indexing_sytem 系统，知道tile上有哪些内容
通过 map tile 索引所有的实体 ，将tile 上的实体添加到tile_content上

8，让player hit things
Bump to attack (walking into the target) is the canonical way to do this. 走到目标的位置
检查玩家走进的tile 是否包含目标
you can walk up to a mob and try to move onto it


9，player attacking and killing things
表示攻击意图的组件，WantsToMelee 
玩家可能遭受多个攻击源，但是Specs 不想将同一个组件多次添加到实体上
所以讲咩一个攻击作为一个实体，要么一个变量存储所有的攻击
选择简单的后一个，SufferDamage component, to track the damage， 并为该组件 实现一个方法 使其易于使用

给玩家添加想要攻击的组件
MeleeCombatSyatem 系统 处理近战, melee 近战 new file melee_combat_system.rs

damage_system 来应用伤害，计算伤害值，new file damage_system.rs

DamageSystem 系统，计算收到的伤害
delete_the_dead 删掉死亡的实体，在tick commmand 中，每一帧都会检测，在系统运行之后

10 让monster hit you back 
只需要为怪物添加WantsToMelee 怪物 就可以攻击玩家

将玩家实体变为资源，这样我们才可以比较容易的引用使用
let player_entity = gs.ecs ... 
gs.ecs.insert(player_entity);

扩展 turn system 
怪物在收到致命伤害后还会继续攻击一次
添加系统状态，
将RunState 添加为资源
从ecs 中得到RunState， 然后根据状态执行逻辑,并修改状态
状态机
灵活使用作用域，将一些只需要使用席次的代码删除，或者只是为了得到数据A，然后将数据A的引用赋值给一个变量
传出去，然后将A删除


--------------------------------------------------------------------
## 2.7 User interface
1,收缩 Map, Shrinking the map,使用常量来设置map 的size
改变 map 的高度 43 ，留下一部分作为user interface

2,some minimal GUI elements 
创建 gui.rs ， draw_ui 在地图下方画一个box 作为UI

3,adding a health bar, 添加生命条，
RLTK provides a convenient helper 从ecs 中获得player 的生命值，然后渲染

4,adding a message log 
添加消息日志，日志作为一种资源，可以被任何系统访问，所有信息都可以告诉你信息

新建文件gamelog.rs，首先对日志进行建模，，struct GameLog
当作资源插入到 ecs 中

5,logging attacks
攻击日志，
change melee_combat_system -> run method

6 notifying of deaths 通知死亡信息死亡日志
修改 damage_system -> delete_the_dead method

6,鼠标支持和工具提示，mouse support,tooltips
鼠标点击地图上的 玩家或者怪物显示 提示

RLTK获取鼠标信息,将鼠标 指向的单元格的背景设置为洋红色

new method gui.rs -> draw_tooltip 
获取 tooltips 所需的组件 names and positions also gets read access to the map itself
检查 鼠标 是否在地图上, 如果不是 退出

if we have any tooltips, look at the mouse position, 如果鼠标的位置在右侧, put the 
tooltips to the right, otherwise to the left

7,optional post-processing 处理 for that truly retro feeling 显示一种复古的感觉
main context.with_post_scanlines

------------------------------------------------------------
## 2.8 items and inventory 物品和库存
在UI中添加　基本物品　拾取　使用 丢弃(drop)

2.8.1 thinking about composing items 组合物品
面向对象 和 实体组件系统的**区别是** 你不是考虑实体的继承，而是什么组件组合成了这个实体

so what makes up an item? 
thinking about it, an item can be said to have the following **properties** 
Renderable, draw it 
Position 
InPack, indicate this item is stored 
Item, which implies that it can be picked up 
if it can be used, the item need some way to indicate that it can be used

2.8.2 consistently random 始终随机
计算机本质上是确定性的 - 因此（无需涉及密码学的东西）当您要求“随机”数字时，您实际上得到的是“非常难以预测序列中的下一个数字”。该序列由种子控制 - 使用相同的种子，您总是会得到相同的骰子
make the RNG random number generator a resource, 作为一种资源，任何系统随时随地访问它
main  ecs.insert(....)

2.8.3 improved spawning 
优化怪物生成，支持生成物品
整理玩家 和 怪物生成代码， 将原来main.rs 中的 玩家和怪物生成代码都放入 spawner.rs


2.8.4 spawn all the things, spawn multiple monster per room,
怪物 物品 在房间内随机生成

2.8.5 health potion(药剂) entities,  
添加组件来帮助定义药水
add Item and Potion components to components.rs,register these in main.rs
add new function spawner ->health_potion
在房间中随机生成随机数量的potion

2.8.6 picking up items, 拾取物品， 
create component **InBackpack**, represent an item being in someone's backpack
玩家和怪物都可以失去物品，他们有一个拾取物品的列表，所以一个 componnent **WantToPickupItem** 来标记，
需要一个系统来处理 WantToPickupItem notices, 所以一个新的文件 inventory_system.rs inventory-库存
添加一个按键 g 拾取物品,add new function palyer.rs ->get_item()
按下G键位如果玩家的位置和物品的位置重合,拾取物品,物品移除 position 组件, 添加 WantsToPickupItem 组件

2.8.7 listing your inventory 列出库存，
列出库存的时候，游戏循环进入另一个状态，
extends main.rs -> RunMode
gui.rs -> show_inventory() gui 显示库存
I 键, 显示库存 inventory.
main.rs -> tick(), we'll add another matchin 添加匹配 ShowInventory
添加 show_inventory() in gui.rs

2.8.8 using items 使用物品
在库存中选中一个item  并使用 
extend the menu to return an item and a result
gui.rs -> show_inventory() gui 物品菜单栏的 按键操作 Escape
RunState::ShowInventory 打印选中物品的名字

玩家和 怪物 都可以使用物品 如 药水
add 意图组件 WantsToDrinkPotion

add PotionUseSystem in inventory_system.rs,this iterates all of the WantsToDrinkPotion intent objects, 然后回复 drinke 一定的生命值 Potion 
由于所有放置信息都附加到药水本身，因此无需四处寻找以确保将其从适当的背包中取出：该实体不再存在，并带走其组件。

使用 cargo run 进行测试会令人惊讶：该药水在使用后并没有被删除！这是因为 ECS 只是将实体标记为 dead - 它不会在系统中删除它们（以免弄乱迭代器和线程）。因此，在每次调用 dispatch(派遣) self.run_systems(); 之后，需要添加对 maintain 的调用。
```rust
RunState::PreRun => {
    self.run_systems(); // dispatch 系统
    self.ecs.maintain();
    newrunstate = RunState::AwaitingInput;
}
```

2.8.9 dropping items 从仓库丢弃物品
遵循 使用物品的模式，**create an intent component**,a meun to select it, and a system to perform the drop
WantsToDropItem components
add ItemDropSystem to the inventory_system  
显示 待丢弃物品的菜单 change the gui.rs, add in ShowDropItems
extend impl GameState for State, RunState::ShowDropItem => {....}

10 render order 渲染的顺序
药水显示在玩家的上方
add render_order filed to Renderable Component
player's render_order is 0
monster's render_order is 1

根据 render_order 进行渲染

change render section in tick method 

*每个项目都有对应的自己的文档*

## 2.9 Ranged Scrolls and Targeting 远程卷轴和目标

last chapter, we added items and inventory - and a single item, a health potion, now a second item type: a scroll of magic missile(魔法导弹卷轴), the lets you zap（攻击） an entiy at range

1，using components to describe what an item does 使用组件来描述项目的功能，组合组件

fot flexibility, we will start by breaking down items into a few more components types 

start with the simple flag component, Consumable component 可消耗的组件

PotionUseSystem -> ItemUseSystem

将Potion 组件 修改为 ProvidsHealing Component, 

change the spawner.rs -> health_potion()

2，describing ranged magic missile（导弹） scrolls, 描述远程魔法导弹卷轴
add more components and registeres in mian.rs, Ranged 范围 InfilctDamage 给予（使遭受）损坏

write magic_missile_sroll function in spawner.rs, describing the scroll 

add magic_missile_sroll function into the spawn list, spawner.rs -> random_item()

将生成物品的生成代码 health_potion 修改为 random_item

now, you'll find scrolls as well as potions lying around, the components system already provides quite a bit of functionality, 

you can see them rendered on the map(thanks to the **renderable** and **positon**)
you can pick them up and drop them(thanks to **item**)
you can list them in your **inventory**
you can call **use** on them, and they are destroyed: but nothing happens

3，implementing ranged damage for items 对物品实施远程伤害
want magic missile to be 可以瞄准，激活发射，然后选中一个 受害者，这是另一种的输入模式，添加运行状态 extend main.rs RunState add ShowTargeting 
extend main.rs -> match newrunstate -> ShowTargeting handle items that are ranged (存在ranged 组件的item ) and include mode switch (模式转换) to ShowTargeting gui绘制攻击选择菜单 gui::ranged_target, 攻击选择菜单, 显示视域, 鼠标选择攻击目标, 返回攻击目标的信息

extend mod.rs RunState::ShowTargeting 匹配

将使用药水的组件修改为WantsToUseItem 组件

将使用potion 系统改为 ItemUseSystem 修改 inventory_system.rs

ItemUseSystem 有使用不同物品的条件，如魔法导弹攻击，药水治疗，AOE攻击等

4 Introducing Area of Effect 引入范围攻击
add another scroll type Fireball, 引入 Aoe 伤害 
add a component AreaOfEffect, 存在攻击半径

extend spawner.rs -> random_item 增加一个item 

write a fireball_scroll function to actually spawn them, like other items

现在火球术 会对单体产生伤害，we must fix that，add new vector storage targets
extend inventory_system.rs ->  match useitem.target {

现在 所有的 物品 的使用目标都是 从对 targets 的迭代进行 获取 for target in targets.iter() {

5 Confusion Srolls 昏迷卷轴
add another item - confusion scrolls, will target a single item at ranged, make them confused for a few turns - during which time thay will do nothing. 
start by describing what we want in the item spawning code. extend spawner.rs -> fn confusion_scroll(ecs: &mut World, x: i32, y: i32) {

add a new Confusion   component (and register it!):

extend ItemUseSystem, add the ability to paas along confusion to the ItemUseSystem

存储 被confusion 的 目标的vector ，let mut add_confusion = Vec::new();

update the **monster_ai_system** to use Confusion status, Confused entity can not act,    if i_anm_confused.turns < 1 只有 Confused 中的 turns 被消耗为0，哪里在消耗这个turns, 怪物每次行动都会检查遍历 confused 存储器，如果存储器中有这个实体，就会  i_am_confused.turns -= 1;


## 2.10 Saving ang Loading 保存和加载
停止游戏然后继续游戏
1 A Main Menu 主菜单
resume a game
a main menu give option to abandon your last save, view credits（制作组)

being in the menu is a state, so extend RunState, include menu state inside it, MainMenu { menu_selection : gui::MainMenuSelection }, handle the new RunState,

in gui.rs, add couple of enum types to handle main menu selections, pub enum MainMenuSelection , pub enum MainMenuResult

handle the new RunState MainMenu, ensure that we are not also rendering the GUI and map when in the menu, rearrange（重新安排） GameState -> tick()

hadle MainMenu state, 处理处于 主菜单 状态 下的逻辑 RunState::MainMenu{ .. } => {}, if something has been selected, change the game state,
for quit, terminate the process, for now, loading/starting a game do the same thing: go into the PreRun state
to setup the game

the last thing to do is to write the menu itself, in gui.rs -> main_menu()

match ctx.key{}, displays menu options and lets you select them with up/down keys and enter, it is very careful to not modify state itself.

2 Including Serde（序列化）

Serde is pretty much the gold-standard for serialization in rust, in Cargo.toml inlcude serde and serde_json

cargo run, it will downloading the new dependencies and all of their dependencies.

3 Adding a SaveGame state
extend RunState add SaveGame
in tick, add RunState::SaveGame => {} 处理SaveState 状态下的逻辑 

in player.rs, add anthor keyboard handler - escape, press escape to quit the menu

4 Getting started with saving the game 开始保存游戏
now that the scaffolding is in place, it is  time to actually save something, 

in the tick function, we extend the save system to just dump(转储) a JSON representation of the map to the console, 将地图的json格式 转储在控制台

need to tell Map to serialize itself, 需要让Map 自己进行序列化, 对 struct Map 添加宏，也需要对 TileType 和 Rect 进行序列化

now when you hit escape it will dump a huge blob of JSON data to the console


地址，开还是断

5 Saving entity state, 保存实体状态
but because of Specs handles Entity 的方式，实体的ID 合成的，不能保证下次会得到相同的ID，
另外，我们可能不想保存所有的内容，所以，引入 specs 中的标记 markes 概念，它提供了一个非常强大的序列化系统

6 Introducing Markers 引入标记
main.rs to make use marker functionality
use specs::saveload::{SimpleMarker, SimpleMarkerAllocator};

in components.rs, add marker type pub struct SerializeMe, in main.rs, add SerializeMe to the list of things that we register

insert a marker entity as ECS resource

in spawner.rs, tell each entity builder to include the marker,.marked::<SimpleMarker<SerializeMe>>() needs to be repeated for all of your spawner in this file
use specs::saveload::{MarkedBuilder,SimpleMarker};

7 The ConverSaveload(转换保存加载) derive macro

Entity 类本身（由 Specs 提供）不能直接序列化， it's actually a **reference** to an identity in a special structure called a "slot map" 

 in order to save and load Entity classes, it becomes necessary to convert these synthetic（合成） identities to unique ID numbers.

Specs provides **a derive macro called ConvertSaveload for this purpose.** It works for most components, but not for all（但是不适合与所有的组件）

序列化一个没有实体但**有数据的类型**非常容易：用 #[derive(Component, ConvertSaveload, Clone)] 标记它。


#[derive(Component, ConvertSaveload, Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}
Clone 表示“这个结构可以在内存中从一个点复制到另一个点”。这对于 Serde 的内部工作是必要的，并且还允许您将 .clone() 附加到对组件的任何引用的末尾 - 并获得它的另一个完美副本

When you have a component with no data, the ConvertSaveload macro doesn't work! so can fall back to the default Serde syntax. Here's a non-data ("tag") class:

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct Player {}

8 Actually saving something
move code for loading and saving into savedload_system.rs 

implementing save_game function, extend saveload_system.rs, 
bulid macro serialize_individually 解决Serde 和Specs 之间 协同工作的问题

creating a new component type - SerializationHelper that stores a copy of the map, then creates a new entity savehepler, and give it the new component SerializationHelper, 

savegame.json file has appeared with your game state in it,

9 Restoring Game State 恢复游戏状态
it is time load the game state

is there a saved game? 是否有被保存的游戏状态可以被加载，in saveload_system.rs, add funciton does_save_exist()

change 游戏加载的 ui 显示

in main.rs, 编写 游戏加载的逻辑

10 Actually loading the game 实际加载游戏
in saveload_system.rs, need another macro deserialize_individually,  serialize_individually 宏几乎相同 - 但相反的过程(反序列化)，并包括一些细微的变化：
extend saveload_system.rs -> load_game function, 将 savegame.json 数据编码 反序列化 ，然后将反序列的结果 转为 specs 组件, 替换 地图 资源，存储玩家和其位置的 世界资源

11 Just add permaddeath 添加永久性的死亡
roguelike 不会在你重新加载游戏后保存你的游戏存档，add delete_save() function to saveload_system.rs

add a call to mod.rs to delete the save after we load the game

12 Web Assembly 网络组装
wasm is sandboxed(沙盒)，does not have the ability to save files locally, 没有能力保存文件到本地

rust offers condition 条件编译，就是 C 语言 中 #define
仅在非 Web 汇编平台上时编译
#[cfg(not(target_arch = "wasm32"))]
pub fn save_game(ecs : &mut World) {}

now have a framework for loadin and saving the game whenever we want to, 
adding components has gained some steps: weh have to register them in main, tag them for Serialize, Deserialize, and add them to our components type lists in saveload_system.rs


## 2.11 Delving Deeper 
dungeon crawler 地牢探索者
this chapter weil introuce depth, with a new dungeon being spawned on each level down, 每个等级 都会生成一个地牢
we will track the player is depth, and encouraged ever-deeper exploration, 
what could possibly go wrong for the player?

1 Indicating - and storing - depth 指示和存储深度
adjust Map struct to include an integer fo depth，给Map添加深度属性

pub fn new_map_rooms_and_corridors(new_depth : i32) -> Map {}, 地图生成器可以创建其他level 的地图，所以为地图生成器函数添加一个参数 new_depth

that is, our maps now konw about depth, you will to delete any savegame.json files you have lying around, since we have changed the format - loading will fail

2 Showing the player their map depth 向玩家显示地图的深度
we will modify the player is heads-up-display to indicate the current map depth, in gui.rs, inside the draw_ui function

3 Adding down stairs 添加向下一层的楼梯
modify the enumeration TileType, add new one:down stairs

render the stairs, map.rs contains draw_map, and adding a tile type is a relatively simple task

lastly, place the down stairs(放置向下向上的楼梯), place up stairs in the center of the first room the map generates, place the stairs in the center of the last room, modify new_map_rooms_and_corridors function in map.rs

now, can find a set of down stairs on the map

4 Actually going down a level, 下降一层
in player.rs, 将下一级的操作绑定到 period 键位上（在美国键盘上，这是不带 Shift 的 > 键）,add new function try_next_level() into player.rs

add new RunState NextLevel, implemented the new RunState, 为新的游戏状态添加对应的逻辑 self.goto_next_level() 函数(State 的一个方法)

add a new impl section for State, so we can attach methods to it, create a new helper method(辅助函数) 
impl State{
    fn entities_to_remove_on_level_change(&mut self)->Vec<Entity>{

    }
}
when we go to the next level, we want to delete all entities - except the player and whatever equipment(装备) the player has, that is what the helper funciotn does

next step, we go to create the goto_next_level function, also inside the State implementation:
删除需要删除的实体 -> 创建一个新的地图 -> 获得当前地图资源的可写引用，获取当前的级别，将新地图的当前 level+1，将地图换为新的地图， -> 使用初始设置中使用的相同代码，生成每个房间的怪物和物品-> 将玩家的放到第一个房间的中心 -> 玩家的Viewshed 组件，因为玩家现在周围的地图已经发生了变化，所以 Viewshed 已经过时，marker it as dirty, let the various system take care of the rest -> give the player a log entry that they have descended to the next level-> obtain the player is health component, if their health is ledd than 50% - boost it to half(将其提高一半) -> -> -> -> ->

您可以下降到一个实际上无限的（它实际上受 32 位整数的大小限制）地牢。已经了解 ECS 如何提供帮助，以及我们的序列化工作如何在我们添加到项目时轻松扩展以包含像这样的新功能。


避免借用和生命周期的问题，创建一个新的是scope，然后在这个scope 中clone, 把这个clong 给 scope 外的一个变量，这个scope结束会自动销毁原始的数据，

git 的使用中，需要先将本地的修改 提交(add commit push) 然后才可以 从远程进行pull


## 2.12 Difficulty
currently, you can advance through multiple dungeon levels, but they all have the same spawns. 没有难度的区别

1 adding a wait key
roguelike 游戏的一个重要战术元素是跳过回合的能力 - 让怪物向你袭来（并且不会受到第一击！）
the bility to skip a turn, 跳过回合，

in player.rs add numeric keypad 5 and space to be skip，implement skip_turn function:
looks up various entities, and then iterates the player is viewshed using the tile_content system, it checks what the player can see for monsters; if no monster is present, it heals the player by 1 hp. 

这为游戏增加了一个很好的战术维度：你可以将敌人引向你，并从战术布局中受益。 Roguelike 游戏的另一个常见功能是等待，如果附近没有敌人，就会提供一些治疗。

2 Increased difficulty as you delve: spawn tables 随着leve 难度变化
怪物和物品的生成更加随机，有些东西常见，有些东西稀有
create a random_table system for use in the spawn system, create new file, random_table.rs, c

struct RandomEntity, 随机的实体，name, weight 权重影响珍惜程度
pub struct RandomTable， 一个向量包含RandomEntity，总体的权重
impl RandomTable -> new method, add method, roll method,

extend spawner.rs, create new function room_table, replace the room spawning code with room_table function

#[allow(clippy::map_entry)]
clippy是Rust的一个静态代码分析工具，用于检查代码中的潜在问题和不良习惯。map_entry警告是指在使用**HashMap**时，应该使用entry方法来插入或更新键值对，而不是使用get方法再进行插入或更新操作。通过在代码中添加#[allow(clippy::map_entry)]注释，可以告诉编译器忽略这个警告，不会对代码进行相关的检查和提示。

simplify a bit, Delete the NUM_MONSTERS, random_monster and random_item functions in spawner.rs, exchange spawn_room function 

1d7-3 (for a -2 to 4 range).

find randome spawn point, add it into the spawn list, then we iterate the spawn list, match on the roll result and spawn monster and items

3 Increasing the spawn rate as you delve 随着探索增加生成率
solve the problem of later levels of being of the same difficulty as earlier ones,
随着 下降 产生更多的实体，start by modifying the funciton signature of **spawn_room** to accept the map depth
根据深度值改变创建的实体的数量

change a couple of calls in main.rs to pass in the depth


4 Increasing the weights by depth, 根据深度增加权重
modify the **room_table function** to include map depth, also change the call to it in spawn_room to use it

we now have a dungeqon that increases in difficulty as you descend, in the next chapter, giving your character some progression as well(through equipment), to balance things out


## 2.13 Equipment 装备
equipping a weapon and shield, 装备武器和盾牌

1 adding some items you can wear/wield
extend spawners.rs, new function dagger(匕首) and shield, 创建匕首 和 盾牌， 比将其添加到生成表中 和 spawn_room 

2 equipping the item 装备物品
2.1 equipable components 可装备的部件
we need a way to indicate that an item can be equipped, add new component EquipmenSlot(装备槽) and Equippable(可装备的) into component.rs 

serialization support it and register it

in saveload_system.rs, add it to serialize and deserialize components lists

add equippable components to dagger and shield function in spawner.rs

2.2 making items equippable 使得物品可以装备
new components Equipped, 表示该物品处理被装备的状态，it will indicate what slot(插槽) is in use，register it in main.rs and include it in the serialization and dserialization lists in saveload_system.rs

2.3 actually equipping the item 实际装备该物品
实际装备该物品到某一个插槽，取消这个插槽中已经有的物品，在使用物品的接口中完成这个功能，open invenotry_system.rs, and we will edit ItemUseSystem, start by expanding the list of system we are reference:
start by matching to see if we can equip the item, 
if we can, it looks up the target slot for the item add looks to see if there is already an item in that slot, if  there, it moves it to the backpack,
lastly, it adds an equipped components to the item entity with the owner(the player right now) and appropriate slot,

when the player moves to the next level we delete a lot of entities, we want to include Equipped by the player as a reason to keep an item in the ECS, 
in main.rs, we modify entitirs_to_remove_on_level_change

2.4 granting combat bonuses 授予战斗奖励
logically, s shield should provide some protection aganist incoming damage - and being stabbed with dagger should hurt more than being punched! to facilitate(促进) this, we will add some components in
components.rs: MeleePowerBonus 近战攻击奖励 and DefenseBonus 防御奖励， remember to register them in main.rs, and saveload_system.rs, we can then modify our code in spawner.rs to add these components to 
the right items dagger and shield

modify the melee_combat_system to apply these bonus, we do this by adding some additional ECS queries to our system:
We've added MeleePowerBonus, DefenseBonus and Equipped readers to the system.
Once we've determined that the attacker is alive, we set offensive_bonus to 0. offensive 攻击
we iterate all entities that have a MeleePowerBonus and Equipped entry, if they are equipped by attacker, we add their power bonus to offensive_bonus
Once we have determined that the defender is alive, we set defensive_bonus to 0.
We iterate all entities that have a DefenseBonus and an Equipped entry. If they are equipped by the target, we add their defense to the defense_bonus.
When we calculate damage, we add the offense bonus to the power side - and add the defense bonus to the defense side.

2.5 unequipping the item 取消装备该物品
you may want to stop holding an item anf retur it to your backpack, bind the R key to remove an item, in player.rs, add this to the input code 
add ShowRemoveItem to RunState in main.rs, add a handler for it in tick 
mplement a new component in components.rs (see the source code for the serialization handler; 
it's a cut-and-paste of the handler for wanting to drop an item, with the names changed):WantsToRemoveItem, has to be registered in main.rs and saveload_system.rs.

in gui.rs, implement remove_item_menu, it is almost exactly the same as the item dropping menu, but changing waht is queries and the heading, it is be a grate idea to make these into more generic
functions some item

extend inventory_system.rs to support removing items, add pub ItemRemoveSystem system, add it to the in main.rs

系统交由 ECS 的调度系统进行执行


3 adding　some more powerful gear later

add couple more items in spawner.rs: longsword, tower_shield, add them to the room_table, with a chance of appearing later in the dungeon(地牢)
to add a quick fix to random_table.rs to ignore entrites with 0 or lower spawn chance(机会)

4 the game over screen 游戏结束画面
we are nearly at the end of the basic tutorial, let is make something happen when you die - rather than locking up in console loop, in the file damage_system.rs, we will edit the match statement 
on player for delete_the_dead, add new state GameOver to RunState
call game_over function to render the death menu, and when you quit we delete everything in the ECS, Lastly, in gui.rs, we will implement game_over function
handle game_over_cleanup:
If you cargo run now, and die - you'll get a message informing you that the game is done, and sending you back to the menu.

# Section 2 - Strech Goals 延伸目标
this short chapter will show to use a bitmask to calculate appropriate 适当的 walls and render them appropriately 适当的

1 counting neighbours to build our bitset
function draw_map, match tile by type, extending the Wall section,new function wall_glyph 通过画线字符绘制墙