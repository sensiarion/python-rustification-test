# Игра жизнь

Проект проба rust'а и его интеграции с питоном, на примере CPU bound задачи.

Де факто, это не такая уж вычислительно сложная задача, но упираемся мы именно в CPU

![Профайлинг pure python версии](./docs/pure%20python%20profile.png)

## Почему игра жизнь

Де факто, сам процесс симуляции игры является нагрузочным бенчмарком. Основная работа - CPU математика (довольно
простая, но всё же), которую достаточно легко вынести отдельно и использовать. Но при этом есть сопуствующие нюансы (
нужно работать с питонячими объектами из раста, есть потенциал в обёртке над pygame в том числе ). А кодовая база не
такая большая и сложная как у прод проектов.

## Реализации

Для каждой из реализации (за исключением `--release` сборок) **создана отдельная ветка**, в которую вы можете
переключиться
для проверки и сравнения производительности.

### Запуск

1. python3.10; `pip install -r ./requirements.txt` + rust 1.79.0
2. `maturin develop` или `maturin develop --release` (для release версий)
3. добавить в `PYTHONPATH` src директорию
4. python src/main.py

Шаг 2 и 4 обязательно повторять после переключения в каждую из веток

### Базовая версия

_Ветка:_ **pure-python**

- pre render поля во избежание доп. затрат на рендер (хоть и не шибко больших)
- самые нагруженные методы - `_get` и `_neighbors` (68% CPU time там)
- render занимает порядке 17% времени
- используют numpy массив для состояния поля

### Точечно оптимизированная

_Ветка:_ **partly-optimized**

- `_get` и `_neighbors` заменены на rust реализацию реализацию
- отдельно рассмотреть влияние inline'инга

------

![первый вариант оптимизации](./docs/first%20rust%20impl%20profile.png)

Штош, при первой реализации перфоманс не особо изменился (даже стал чуть хуже).

Почему производительность не поменялась и даже ухудшилась:

- ndarray для хранения был заменён на `list[list[Cell]]`, т.к. я хз какой тип у нампаевского массива, а точнее как его
  скатить. В теории есть крейт для numpy массивов, но это отдельная история (да и тут назревают оптимизации получше)
- abi стандарт принуждает нас особым образом де/серриализовать (скорее маршалинг, но не суть) данные, что создаёт
  накладных расходов больше чем питонячая математика
- как ни странно, оверхед на вызов питонячей функции не сильно ощущается (хотя в `neighbors` вызов `get_from_field` уже
  зашит во внутрь, так что не удивительно). По крайней мере вынос `neighbors` на прямой вызов (вместо одноимённой
  функции обёртки), значимых изменений не дал

Круто было бы задизасемблить библиотеку и посмотреть как оно внутри, в частности, работает ли инлайн (судя по тому что я
позже увидел в https://youtu.be/eDZHEkKZXuU?si=2bBJ6lCwykHD6peI&t=1132 работать он должен, т.к. у нас не `abi3`).

Если не считать совокупление с типами pyo3 (даже примеров нормальных не увидел, долго тыкался), то сама логика уже
пишется без проблем (никаких внизапных вылетов и мудрёных концепций обработки в коде. Если компилиться - то работает. А
ещё в дебаг режиме вылетающие ошибки раста реально понятные, без всяких segfault).

### Точечно оптимизированная (--release flag)

_Ветка:_ **partly-optimized**

![первый вариант оптимизации](./docs/first%20rust%20impl%20profile%20with%20release%20flag.png)

Как вы догадались из названия, просто добавился релиз флаг, но перфоманс вырос значительно (на моём mac m1 pro в debug
сборке было порядка 3.9 fps после заполнения карты, а в release сборке порядка 7.9 (и тот и другой с запущенным
cprofile, снапшот которого представлен выше)).

Это уже приятнее (я не ожидал такого буста, думал маршалинг всё сожрёт)

### Заменён update с игровой логикой

_Ветка:_ **update-optimized**

- используется всё то же ~~numpy~~ стандартное поле на списках
- дополнительно заменена логика (вынесен `update`)
- рендер всё ещё идёт по питонячему iterate

-----------

_Дебаг сборка_

![update optimized profile](docs/update optimized profile.png)

_Релиз сборка_

![update optmized release profile](docs/update optmized release profile.png)

В дебаг сборке порядка 7.9 кадра в секунду, в релиз сборке порядка 11.

Проблем при реализации не встречено, тут всё быстренько по аналогии получилось. Отдельно отмечу супер приятные ошибки,
которые отдаёт раст в дебаг режиме. Всё супер понятно, отлаживать на пару порядков удобнее чем взаимодействие с
плюсами.Да и в целом оно в разы приятнее, никаких биндингов, ничего писать не нужно.

Как видим, рендер уже начинает отжирать порядочно.

### Rust абстракция для поля

_Ветка:_ **field-rust-impl**

- Всё что касается работы с полем (и итерирование и само хранение), перенесено в rust
    - наружу доступен `iterate` для накидывания рендера

------------

_Дебаг сборка_ (~8фпс)

![field-rust-impl prifle debug](docs/field-rust-impl prifle debug.png)

_Релиз сборка_ (~34фпс)

![field-rust-impl prifle release](docs/field-rust-impl prifle release.png)

Вот тут уже стало лучше. Даже с учётом кучи копирований в расте (я там далеко не идеальный код написал), перфоманс таки
вполне заметен. Как мы видим, теперь почти всё время у нас занимает рендер, а не наша логика игры.

Данное переписывание далось уже посложнее (то borrow checker цапнет, то ищешь примеры вызова).

Как минимум, не стоит перекидывать между питоном и растом, куда проще замкнуть логику непосредственно в расте и работать
там. В частности пришлось переписать `_get -> Cell`, на `get_state` и `set_state` соответственно (потому что выдать
атомарное значение клетки с поля можно только при его копировании, а делать это, например, в update не очень то
хотелось). В теории это можно было обойти, но нужно больше скила (простор для рефакторинга).

### Заменена логика рендера

_Ветка:_ **render-optimzied**

- логика рендера у `LiveWorld` перенесена в rust

----------

_Дебаг сборка_ (~22фпс)

![render-optimzied debug](docs/render-optimzied debug.png)

_Релиз сборка_ (~117фпс)

![render-optimzied release](docs/render-optimzied release.png)

Де факто куча времени теперь уходит на вызов pygame методов через питон. В теории, можно попробовать вызывать SDL2
методы напрямую (биндинги для раста есть), но это потребует использования куда большей магии (да и для удовлетворения
изначальных целей проекта бесполезно).

## Выводы

### Вопросы, волновавшие до реализации

- насколько сложно интегрировать rust
- как тяжело работать с питонячими объектами
    - в плюсах даже для возвращения null объектов нужны были сакральные знания иначе segfault
- как тяжело работать с внешними библиотеками (numpy и pygame - обвзяки поверх C++, нужно постораться ходить до них в
  обход питон обвязки, по возможности)
- можно ли внедрить в существующий пайплайн разработки наших проектов (у нас есть `poe configure`, как entrypoint
  проекта)
    - сложность сборки исходников под каждой из OS (достаточно ли скачать rust и запускать скриптик при развёртывании
      проекта или нужен бубен)

### Результаты

Выводы по перфомансу:

- постоянное таскание данных из питона в rust почти нивелирует прирост производительности от вычислений
- если у вас есть внешняя зависимость (другая нативная библиотека, которую вы вызываете через питон или же необходимость
  вызывать реализованные на питоне хуки), которую вы вызываете в большом кол-ве, то вас перфоманс опять же сожрут
- дебаг сборки сильно съедают производительность (но это большая заслуга pyo3 и его безопасных обвязок для вывода
  хороших ошибок)
- Даже при всех проблемах, оптимизация в 30 раз очень даже результат

Для замеров на графиках выполнялся запуск через `cprofile` (без него производительность будет ещё выше), значение fps
взято после заполнения всей доски элементами.

![fps_summary](docs/fps_summary.png)

![img.png](docs/cpu_spent_render_summary.png)

![cpu_logic_summary](docs/cpu_logic_summary.png)

Под % CPU имеется в виду CPU time в рамках запущенного процесса, о утилизации ядер речь не идёт.

Де факто, это далеко не потолок производительности. Помимо этого, можно было бы:

- попробовать завезти SIMD инструкции и иные варианты параллелизма
- отрефакторить код для уменьшения кол-ва операций копирования
- использовать на первых этапах оптимизации numpy массив, как и в pure python версии (в идеале, через обвязки numpy в
  rust, но я хз как это делается (скорее всего нужно выхватить из питон объекта сырой указатель и скастить через
  unsafe))

### Про разработку и её удобство

Разработка получилась очень даже приятной (почти не было дебага):

- если в коде есть ошибка, то код просто не соберётся
- большая часть ошибок была связана с отсутствием сигнатур и типизации (при вызове из питона)
    - никакого бойлерплейта; накинул декоратор на функцию и она уже вызывается из питона. Разве что `pyi` файлы
      автоматом не генерятся, но и над этим уже идёт [работа](https://github.com/PyO3/pyo3/issues/2454)
    - для сравнения, обвязки на плюсах требовали бы бубнов в виде extern C, затем бы ещё пришлось писать описания
      функций, а любые ошибки там кончаются segfault'ами, которые придётся долго курить
        - и ещё стоит учесть, что там имеется ворох соглашений по обработке ошибок, вызову кода и возвращению значений.
          На расте же просто пишутся обычные функции, ровно так, как это делалось бы и без интеграции с питоном
- если ошибка и обнаруживалась в расте, то либо это была ошибка логики, либо ты получаешь реально понятный трейсбек с
  ошибкой (и то, ошибку я получил при вызове питонячьего метода из раста, не указав нужные аргументы)
  - **UPD:** он такой даже в релиз моде, ляпота
- С компилятором по началу приходится бороться (но чем больше кода писалось и чем больше сниппетов появлялось, тем
  больше это отходило на второй план)
- Совокупное время доработок по расту вышло в районе 8 часов (от момента начала чтения доки по вариантам интеграции, до
  окончания описания этой доки). Из этого времени субъективно:
    - написание доки, скриншоты и др. - _3ч_
    - первый запуск обвязки - _30м_
    - написание методов - _2ч_ (я просто слоупок который слабо знает даже базу раста)
    - борьба с компилятором (рефакторинг, поиск вариантов решения проблем с borrow check'ером или обвязками pyo3) -
      _2ч30м_

### Туториал по добавлению rust в python проекты

- `pip install maturin` для CLI обёртки над rust
- `maturin init` для инициализации rust файлов (в корневой проекта, `где pyproject.toml`, может его зашакалить, нужно
  быть внимательным)
- переименовать директорию в `rust_src` (src по умолчанию)
- поменять путь до файлов библиотеки в `Cargo.toml`
    - ```toml
  [lib]
  path = "rust_src/lib.rs"
    ```
- `maturing develop` для dev сборки библиотеки (или же `maturing develop --release` для сборки release версии)
    - после вызова будет собран проект (то же самое, что `poetry install --only-root`)
    - rust функции будут доступны для имопрта из проекта (`live_game.sum_as_string`)
        - одна беда, типизация не прокидывается сама
          собой ![пример вызова доступного после сборки](docs/build_rust_lib_import_example.png)
        - Но это поправимо https://pyo3.rs/v0.22.2/function/signature
- ??? Как получать сигнатуры функций (pycharm их показывает, но кэширует внутри до момента сброса индекса, а это
  довольно муторный процесс)
    - **UPD:** проблема не решена автоматом, но
        - для уже готовой библиотеки у разрабов будут подтягивиться корректные описания доступных объектов и методов (но
          не сигнатуры)
        - сигнатуры можно без проблем дописывать самим через pyi файлы

### Проблемы

- jetbrains не соизволила завести поддержку rust'а для pycharm в виде плагина, нужна отдельная IDE (RustRover)
- в случае поставки исходников без бинарников разрабам придётся ставить раст на свои машины
    - однако наладить авто сборку не сложно (при инициализации проекта вообще идёт скрипт для github CI, который
      собирает под все самые популярные платформы на публичных раннерах)
    - в нашем случае столь же не солжно добавить сборку под виндой линуксом и маком
    - C++ компиляторы ведь почти у всех стоят, поставить растовский также будет не сложно
    - можно решить проблему добавлением компилятора раста в наши базовые образы сборки