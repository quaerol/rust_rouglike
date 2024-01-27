
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

### 2.1 实体和组件
### 2.4. Field of View
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

### 2.5 monster
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


### 2.6 Dealing Damage
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

### 2.9 Ranged Scrolls and Targeting 远程卷轴和目标

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
extend main.rs -> match newrunstate -> ShowTargeting handle items that are ranged (存在ranged 组件的item ) and include mode switch (模式转换) to ShowTargeting gui绘制攻击选择菜单 gui::ranged_target



git 的使用中，需要先将本地的修改 提交(add commit push) 然后才可以 从远程进行pull
