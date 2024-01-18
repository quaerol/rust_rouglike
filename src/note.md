### 2.6 Dealing Damage
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

7，map 的 tile_content 存储tile 上的内容，

让player hit things
索引所有的实体通过tile ，将tile 上的实体添加到tile_content上

player attacking and killing things
表示攻击意图的组件，WantsToMelee 
玩家可能遭受多个攻击源，但是Specs 不想将同一个组件多次添加到实体上
所以讲咩一个攻击作为一个实体，要么一个变量存储所有的攻击
选择简单的后一个，SufferDamage component
给玩家添加想要攻击的组件
MeleeCombatSyate, melee 近战攻击组件
DamageSystem 系统，计算收到的伤害
delete_the_dead 删掉死亡的实体，在tick commmand 中，每一帧都会检测，在系统运行之后

让monster hit you back 
怪物添加WantsToMelee
将玩家实体变为资源，这样我们才可以引用使用
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
### 2.7 User interface
1,收缩 Map, Shrinking the map,使用常量来设置map 的size
2, 改变 map 的高度，留下一部分作为user interface
3,创建 gui.rs ， 在地图下方画一个box 作为UI
4,添加生命条，从ecs 中获得player 的生命值，然后渲染
5,添加消息日志，日志作为一种资源，可以被任何系统访问，
首先对日志进行建模，新建文件gamelog.rs，struct GameLog
当作资源插入到 ecs 中
攻击日志，死亡日志

6,鼠标支持和工具提示，mouse support,tooltips
鼠标点击地图上的 玩家或者怪物显示 提示

7,optional post-processing for that truly retro feeling

------------------------------------------------------------
### 2.8 items and inventory 物品和库存
在UI中添加　基本物品　拾取　使用 丢弃(drop)
2.8.1 composing items 组合物品
面向对象 和 实体组件系统的区别是 你不是考虑实体的继承，而是什么
组件组合成了这个实体
so what makes up an item? thinking about it, an item can be
said to have the following properties
Renderable, draw it 
Position 
InPack, indicate this item is stored
Item, which implies that it can be picked up
item need some way to indicate that it can be used
2.8.2 consistently random 始终随机
计算机本质上是确定性的 - 因此（无需涉及密码学的东西）当您要求“随机”数字时，您实际上得到的是“非常难以预测序列中的下一个数字”。该序列由种子控制 - 使用相同的种子，您总是会得到相同的骰子
make the RNG random number generator a resource, 作为一种资源，我们随时随地访问它
2.8.3 improved spawning 优化怪物生成，支持生成物品
整理玩家 和 怪物生成代码， 将他们都放入 spawner.rs
2.8.4 spawn all the things, spawn multiple monster per room,
2.8.5 health potion(药剂) entities,  add Item and Potion components to components.rs,register these in main.rs
在房间中随机生成随机数量的potion
2.8.6 picking up items, 拾取物品， create component InBackpack, represent an item being in someone's backpack
玩家和怪物都可以失去物品，他们有一个拾取物品的列表，所以一个 componnent WantToPickupItem 来标记，
需要一个系统来处理 WantToPickupItem notices, 所以一个新的文件 inventory_system.rs inventory-库存
添加一个按键 g 拾取物品,get_item()

2.8.7 listing your inventory 列出库存，
游戏循环的另一种状态，列出库存的时候，游戏循环进入另一种，其他系统停止运行
gui -> show_inventory() gui 显示库存
2.8.8 using items 使用物品
*在库存中选中一个item  并使用 
2.8.9 dropping items 丢弃物品
遵循 使用物品的模式，create an intent component,a meun to select it, and a system to perform the drop
10 render order 渲染的顺序
药水显示在玩家的上方
------------------------------------------------------------
先写出伪代码 ，一步一步做什么，然后将伪代码翻译成 真正的代码
