using System;
using System.Collections.Generic;
using System.Linq;
using UnityEngine;

namespace Dorfromantik.Area
{
	// Token: 0x020003E6 RID: 998
	public class Area : MonoBehaviour
	{
		// Token: 0x140000C7 RID: 199
		// (add) Token: 0x06001715 RID: 5909 RVA: 0x0006582C File Offset: 0x00063A2C
		// (remove) Token: 0x06001716 RID: 5910 RVA: 0x00065864 File Offset: 0x00063A64
		internal event Action<Area> OnAreaCompletion;

		// Token: 0x170002A7 RID: 679
		// (get) Token: 0x06001717 RID: 5911 RVA: 0x00065899 File Offset: 0x00063A99
		// (set) Token: 0x06001718 RID: 5912 RVA: 0x000658A1 File Offset: 0x00063AA1
		internal int AreaSlotCapacity
		{
			get
			{
				return this.areaSlotCapacity;
			}
			private set
			{
				this.areaSlotCapacity = value;
			}
		}

		// Token: 0x170002A8 RID: 680
		// (get) Token: 0x06001719 RID: 5913 RVA: 0x000658AA File Offset: 0x00063AAA
		// (set) Token: 0x0600171A RID: 5914 RVA: 0x000658B2 File Offset: 0x00063AB2
		internal AreaType Type
		{
			get
			{
				return this.type;
			}
			set
			{
				this.type = value;
			}
		}

		// Token: 0x170002A9 RID: 681
		// (get) Token: 0x0600171B RID: 5915 RVA: 0x000658BB File Offset: 0x00063ABB
		// (set) Token: 0x0600171C RID: 5916 RVA: 0x000658C3 File Offset: 0x00063AC3
		internal AreaScope Scope
		{
			get
			{
				return this.scope;
			}
			private set
			{
				this.scope = value;
			}
		}

		// Token: 0x170002AA RID: 682
		// (get) Token: 0x0600171D RID: 5917 RVA: 0x000658CC File Offset: 0x00063ACC
		// (set) Token: 0x0600171E RID: 5918 RVA: 0x000658D4 File Offset: 0x00063AD4
		internal AreaSpawnBehavior SpawnBehavior
		{
			get
			{
				return this.spawnBehavior;
			}
			private set
			{
				this.spawnBehavior = value;
			}
		}

		// Token: 0x170002AB RID: 683
		// (get) Token: 0x0600171F RID: 5919 RVA: 0x000658DD File Offset: 0x00063ADD
		// (set) Token: 0x06001720 RID: 5920 RVA: 0x000658E5 File Offset: 0x00063AE5
		internal List<AreaSlot> AreaSlots
		{
			get
			{
				return this.areaSlots;
			}
			private set
			{
				this.areaSlots = value;
			}
		}

		// Token: 0x170002AC RID: 684
		// (get) Token: 0x06001721 RID: 5921 RVA: 0x000658EE File Offset: 0x00063AEE
		// (set) Token: 0x06001722 RID: 5922 RVA: 0x000658F6 File Offset: 0x00063AF6
		internal List<AreaSlot> EdgeAreaSlots
		{
			get
			{
				return this.edgeAreaSlots;
			}
			private set
			{
				this.edgeAreaSlots = value;
			}
		}

		// Token: 0x06001723 RID: 5923 RVA: 0x000658FF File Offset: 0x00063AFF
		private void Awake()
		{
			this.AreaSlots = new List<AreaSlot>();
			this.EdgeAreaSlots = new List<AreaSlot>();
			this.placedTiles = new List<Tile>();
			if (this.tileOutliner == null)
			{
				this.tileOutliner = base.GetComponent<TileOutliner>();
			}
		}

		// Token: 0x06001724 RID: 5924 RVA: 0x0006593C File Offset: 0x00063B3C
		internal void Initialize(int areaSlotCapacity, AreaType areaType, AreaScope areaScope, AreaSpawnBehavior areaSpawnBehavior, Material areaPreviewColor, string gameObjectName = null)
		{
			if (gameObjectName != null)
			{
				base.gameObject.name = gameObjectName;
			}
			this.previewMaterial = areaPreviewColor;
			this.AreaSlotCapacity = areaSlotCapacity;
			this.tilesNeededForCompletion = this.AreaSlotCapacity * Mathf.Clamp(this.completionPercentageNeeded, 0, 100) / 100;
			this.Type = areaType;
			this.Scope = areaScope;
			this.SpawnBehavior = areaSpawnBehavior;
			this.completionPercentageNeeded = areaSpawnBehavior.completionPercentageNeeded;
		}

		// Token: 0x06001725 RID: 5925 RVA: 0x000659AC File Offset: 0x00063BAC
		internal void Terminate(bool shouldDestroyAreaSlots = false, bool shouldDestroyAreaSignpost = false)
		{
			if (Enumerable.Any<AreaSlot>(this.AreaSlots))
			{
				foreach (AreaSlot areaSlot in this.AreaSlots)
				{
					if (areaSlot.placedTile != null)
					{
						Debug.LogWarning(string.Concat(new string[]
						{
							"The ",
							base.name,
							" is terminated, although there is still a placed tile (",
							areaSlot.placedTile.name,
							") placed on its position."
						}));
					}
					areaSlot.UpdateAreaSlotNeighborsNeighborList(shouldDestroyAreaSlots);
					if (shouldDestroyAreaSlots)
					{
						Object.Destroy(areaSlot.gameObject);
					}
				}
				this.EdgeAreaSlots.Clear();
				this.AreaSlots.Clear();
			}
			Object.Destroy(base.gameObject);
		}

		// Token: 0x06001726 RID: 5926 RVA: 0x00065A8C File Offset: 0x00063C8C
		internal void AddAreaSlot(AreaSlot areaSlotToAdd)
		{
			if (!this.AreaSlots.Contains(areaSlotToAdd))
			{
				this.AreaSlots.Add(areaSlotToAdd);
				if (areaSlotToAdd.IsLocalEdgeAreaSlot)
				{
					this.AddEdgeAreaSlot(areaSlotToAdd);
					return;
				}
			}
			else
			{
				Debug.Log(string.Format("The areaSlot {0} is already added to {1}!", areaSlotToAdd, base.name));
			}
		}

		// Token: 0x06001727 RID: 5927 RVA: 0x00065AD9 File Offset: 0x00063CD9
		internal void AddEdgeAreaSlot(AreaSlot edgeAreaSlotToAdd)
		{
			if (!this.EdgeAreaSlots.Contains(edgeAreaSlotToAdd))
			{
				this.EdgeAreaSlots.Add(edgeAreaSlotToAdd);
			}
		}

		// Token: 0x06001728 RID: 5928 RVA: 0x00065AF5 File Offset: 0x00063CF5
		internal void RemoveEdgeAreaSlot(AreaSlot edgeAreaSlotToRemove)
		{
			if (this.EdgeAreaSlots.Contains(edgeAreaSlotToRemove))
			{
				this.EdgeAreaSlots.Remove(edgeAreaSlotToRemove);
			}
		}

		// Token: 0x06001729 RID: 5929 RVA: 0x00065B12 File Offset: 0x00063D12
		internal void AddPlacedTile(Tile tile)
		{
			if (this.placedTiles.Contains(tile))
			{
				return;
			}
			this.placedTiles.Add(tile);
			if (!tile.IsInitialTile)
			{
				this.CheckForCompletion();
			}
		}

		// Token: 0x0600172A RID: 5930 RVA: 0x00065B40 File Offset: 0x00063D40
		private void CheckForCompletion()
		{
			this.tilesNeededForCompletion = Mathf.RoundToInt((float)this.AreaSlots.Count / 100f * (float)this.completionPercentageNeeded);
			float num = (float)this.placedTiles.Count / (float)this.tilesNeededForCompletion * 100f;
			this.completionPercentageCurrent = Mathf.Clamp(num, 0f, 100f);
			if (this.placedTiles.Count < this.tilesNeededForCompletion)
			{
				this.isFullyComplete = false;
				this.isPercentageComplete = false;
				return;
			}
			if (!this.isPercentageComplete)
			{
				Action<Area> onAreaCompletion = this.OnAreaCompletion;
				if (onAreaCompletion != null)
				{
					onAreaCompletion.Invoke(this);
				}
			}
			this.isPercentageComplete = true;
			if (this.placedTiles.Count >= this.AreaSlots.Count)
			{
				this.isFullyComplete = true;
			}
		}

		// Token: 0x0600172B RID: 5931 RVA: 0x00065C08 File Offset: 0x00063E08
		internal void DrawOutline()
		{
			if (this.Scope == AreaScope.Global && this.tileOutliner.offset < 0f)
			{
				this.tileOutliner.offset = Math.Abs(this.tileOutliner.offset);
			}
			else
			{
				this.tileOutliner.offset = this.tileOutliner.offset * -1f;
			}
			this.tileOutliner.Outline(Enumerable.ToList<IOutlineable>(this.EdgeAreaSlots));
		}

		// Token: 0x0600172C RID: 5932 RVA: 0x00065C7F File Offset: 0x00063E7F
		internal void ClearOutline()
		{
			this.tileOutliner.ClearOutlines();
		}

		// Token: 0x0600172D RID: 5933 RVA: 0x00065C8C File Offset: 0x00063E8C
		private void OnDestroy()
		{
			this.areaSignpost.Terminate();
		}

		// Token: 0x0600172E RID: 5934 RVA: 0x00065C9C File Offset: 0x00063E9C
		private void ShowEdgeAreaSlots()
		{
			AreaSlot areaSlot = Enumerable.FirstOrDefault<AreaSlot>(this.EdgeAreaSlots);
			Material material = new Material((areaSlot != null) ? areaSlot.GetComponentInChildren<Renderer>().sharedMaterial : null);
			material.color = Random.ColorHSV(0f, 1f, 0.7f, 1f, 0.7f, 1f, 0.45f, 0.55f);
			foreach (AreaSlot areaSlot2 in this.EdgeAreaSlots)
			{
				areaSlot2.GetComponentInChildren<Renderer>().sharedMaterial = material;
			}
		}

		// Token: 0x0600172F RID: 5935 RVA: 0x00065D48 File Offset: 0x00063F48
		private void HideEdgeAreaSlots()
		{
			foreach (AreaSlot areaSlot in this.EdgeAreaSlots)
			{
				areaSlot.GetComponentInChildren<Renderer>().sharedMaterial = this.initialAreaSlotMaterial;
			}
		}

		// Token: 0x0400178C RID: 6028
		[SerializeField]
		private AreaType type;

		// Token: 0x0400178D RID: 6029
		[SerializeField]
		internal AreaScope scope;

		// Token: 0x0400178E RID: 6030
		[SerializeField]
		private AreaSpawnBehavior spawnBehavior;

		// Token: 0x0400178F RID: 6031
		[SerializeField]
		private bool isPercentageComplete;

		// Token: 0x04001790 RID: 6032
		[SerializeField]
		private bool isFullyComplete;

		// Token: 0x04001791 RID: 6033
		[SerializeField]
		private float completionPercentageCurrent;

		// Token: 0x04001792 RID: 6034
		[SerializeField]
		private int completionPercentageNeeded;

		// Token: 0x04001793 RID: 6035
		[SerializeField]
		private int tilesNeededForCompletion;

		// Token: 0x04001794 RID: 6036
		[SerializeField]
		private int areaSlotCapacity;

		// Token: 0x04001795 RID: 6037
		[SerializeField]
		private List<Tile> placedTiles;

		// Token: 0x04001796 RID: 6038
		[SerializeField]
		private List<AreaSlot> areaSlots;

		// Token: 0x04001797 RID: 6039
		[SerializeField]
		private List<AreaSlot> edgeAreaSlots;

		// Token: 0x04001798 RID: 6040
		[SerializeField]
		private Material initialAreaSlotMaterial;

		// Token: 0x04001799 RID: 6041
		internal Material previewMaterial;

		// Token: 0x0400179A RID: 6042
		internal AreaSignpost areaSignpost;

		// Token: 0x0400179B RID: 6043
		private TileOutliner tileOutliner;
	}
}

using System;
using System.Collections.Generic;
using System.Linq;
using TMPro;
using UnityEngine;

namespace Dorfromantik.Area
{
	// Token: 0x020003E7 RID: 999
	public class AreaDebugger : MonoBehaviour
	{
		// Token: 0x06001731 RID: 5937 RVA: 0x00065DA4 File Offset: 0x00063FA4
		private void Start()
		{
			this.previewAreaGenerator = this.areaManager.GetComponent<PreviewAreaGenerator>();
			this.areaGenerator = this.areaManager.GetComponent<AreaGenerator>();
		}

		// Token: 0x06001732 RID: 5938 RVA: 0x00065DC8 File Offset: 0x00063FC8
		private void Update()
		{
			if (Input.GetKeyDown(256))
			{
				Debug.Log("Nothing to debug.");
			}
			if (Input.GetKeyDown(257))
			{
				this.areaManager.CreatePreviewAreas(null);
				this.ColorizePreviewAreasRandomly();
			}
			if (Input.GetKeyDown(258))
			{
				this.DisplayGridPosForAllAreaSlots();
			}
			if (Input.GetKeyDown(260))
			{
				this.DisplayAreaNames();
			}
		}

		// Token: 0x06001733 RID: 5939 RVA: 0x00065E30 File Offset: 0x00064030
		private void ColorizeSegments()
		{
			foreach (KeyValuePair<AreaSlot, List<AreaSlot>> keyValuePair in this.previewAreaGenerator.segmentByEdgeAreaSlot)
			{
				Material material = new Material(Shader.Find("Universal Render Pipeline/Lit"))
				{
					color = Random.ColorHSV()
				};
				foreach (AreaSlot areaSlot in keyValuePair.Value)
				{
					areaSlot.GetComponentInChildren<Renderer>().sharedMaterial = material;
				}
			}
		}

		// Token: 0x06001734 RID: 5940 RVA: 0x00065EE4 File Offset: 0x000640E4
		private void ColorizePreviewAreasRandomly()
		{
			foreach (Area area in this.areaManager.LocalPreviewAreas)
			{
				Material material = new Material(Shader.Find("Universal Render Pipeline/Lit"))
				{
					color = Random.ColorHSV()
				};
				foreach (AreaSlot areaSlot in area.AreaSlots)
				{
					areaSlot.GetComponentInChildren<Renderer>().sharedMaterial = material;
				}
			}
		}

		// Token: 0x06001735 RID: 5941 RVA: 0x00065F94 File Offset: 0x00064194
		private void DisplayGridPosForAllAreaSlots()
		{
			foreach (AreaSlot areaSlot in Enumerable.Where<AreaSlot>(this.areaManager.GlobalPlayableArea.AreaSlots, (AreaSlot x) => x != null))
			{
				Object.Instantiate<TextMeshPro>(this.textPrefab, areaSlot.transform).text = areaSlot.GridPos.ToString();
			}
			foreach (AreaSlot areaSlot2 in Enumerable.Where<AreaSlot>(this.areaManager.GlobalPreviewArea.AreaSlots, (AreaSlot x) => x != null))
			{
				Object.Instantiate<TextMeshPro>(this.textPrefab, areaSlot2.transform).text = areaSlot2.GridPos.ToString();
			}
		}

		// Token: 0x06001736 RID: 5942 RVA: 0x000660C0 File Offset: 0x000642C0
		private void DisplayAreaNames()
		{
			if (this.areaManager.LocalPreviewAreas.Count <= 0)
			{
				return;
			}
			foreach (Area area in this.areaManager.LocalPreviewAreas)
			{
				AreaSlot areaSlot = Enumerable.FirstOrDefault<AreaSlot>(area.AreaSlots);
				TextMeshPro textMeshPro = Object.Instantiate<TextMeshPro>(this.textPrefab, areaSlot.transform);
				textMeshPro.text = Enumerable.Last<char>(area.name).ToString();
				textMeshPro.fontSize = 120f;
			}
		}

		// Token: 0x0400179D RID: 6045
		[SerializeField]
		private AreaManager areaManager;

		// Token: 0x0400179E RID: 6046
		private List<Area> previewAreas;

		// Token: 0x0400179F RID: 6047
		private PreviewAreaGenerator previewAreaGenerator;

		// Token: 0x040017A0 RID: 6048
		private AreaGenerator areaGenerator;

		// Token: 0x040017A1 RID: 6049
		[SerializeField]
		private TextMeshPro textPrefab;
	}
}

using System;
using System.Collections.Generic;
using System.Linq;
using UnityEngine;

namespace Dorfromantik.Area
{
	// Token: 0x020003E9 RID: 1001
	public class AreaGenerator : MonoBehaviour
	{
		// Token: 0x0600173C RID: 5948 RVA: 0x00066174 File Offset: 0x00064374
		private void Awake()
		{
			if (this.areaManager == null)
			{
				this.areaManager = base.GetComponent<AreaManager>();
			}
			if (this.previewAreaGenerator == null)
			{
				this.previewAreaGenerator = base.GetComponent<PreviewAreaGenerator>();
			}
		}

		// Token: 0x0600173D RID: 5949 RVA: 0x000661AC File Offset: 0x000643AC
		internal void GenerateInitialAreas()
		{
			Area area = this.SetupArea(this.defaultAreaSpawnBehavior, AreaType.Playable, AreaScope.Global, "globalPlayableArea", null);
			Area area2 = this.SetupArea(this.defaultAreaSpawnBehavior, AreaType.Preview, AreaScope.Global, "globalPreviewArea", null);
			this.areaManager.InitializeGlobalAreas(area, area2);
			Area area3 = this.SetupArea(this.defaultAreaSpawnBehavior, AreaType.Playable, AreaScope.Local, "initialPlayableArea", null);
			foreach (Tile tile in Enumerable.Where<Tile>(Object.FindObjectsOfType<Tile>(), (Tile x) => x.IsInitialTile))
			{
				Vector2Int vector2Int = GridCalculator.WorldToGridPos(tile.transform.position);
				this.SetupAreaSlot(area3, vector2Int, this.defaultAreaSlotMaterial, true);
				area2.AddPlacedTile(tile);
				area3.AddPlacedTile(tile);
			}
			this.CreateInitialAreaSlots(area3);
		}

		// Token: 0x0600173E RID: 5950 RVA: 0x000662A0 File Offset: 0x000644A0
		internal Dictionary<List<AreaSlot>, Area> CreatePreviewAreas(AreaSpawnBehavior spawnBehavior, List<List<AreaSlot>> edgeAreaSlotSegments)
		{
			Dictionary<List<AreaSlot>, Area> dictionary = new Dictionary<List<AreaSlot>, Area>();
			for (int i = 0; i < edgeAreaSlotSegments.Count; i++)
			{
				List<AreaSlot> list = edgeAreaSlotSegments[i];
				string text = string.Format("{0} {1} {2} #{3}", new object[]
				{
					AreaScope.Local,
					AreaType.Preview,
					"Area",
					i
				});
				Material material = new Material(this.defaultAreaSlotMaterial);
				Color color = Random.ColorHSV(0f, 1f, 0.65f, 0.85f, 0.7f, 0.9f, 0.45f, 0.55f);
				material.color = color;
				Area area = this.SetupArea(spawnBehavior, AreaType.Preview, AreaScope.Local, text, material);
				dictionary.Add(list, area);
			}
			this.CreatePreviewAreaSlots(dictionary, spawnBehavior);
			return dictionary;
		}

		// Token: 0x0600173F RID: 5951 RVA: 0x0006636C File Offset: 0x0006456C
		private Area SetupArea(AreaSpawnBehavior spawnBehavior, AreaType areaType, AreaScope areaScope, string gameObjectName = null, Material previewMaterial = null)
		{
			if (previewMaterial == null)
			{
				previewMaterial = this.defaultAreaSlotMaterial;
			}
			Area area = Object.Instantiate<Area>(this.areaPrefab, Vector3.zero, Quaternion.identity);
			int num = Random.Range(spawnBehavior.tilesCountMinMax.x, spawnBehavior.tilesCountMinMax.y);
			area.Initialize(num, areaType, areaScope, spawnBehavior, previewMaterial, gameObjectName);
			SceneOrganizer.Instance.SortInContainer(area);
			this.areaManager.RememberLocalArea(area);
			return area;
		}

		// Token: 0x06001740 RID: 5952 RVA: 0x000663E4 File Offset: 0x000645E4
		private void CreateInitialAreaSlots(Area area)
		{
			for (int i = 0; i < area.SpawnBehavior.totalSpawnIterations; i++)
			{
				List<AreaSlot> list = new List<AreaSlot>(area.EdgeAreaSlots);
				if (this.TrySpawnAreaSlotsAroundPlacedOnes(list, area) && area.AreaSlots.Count >= area.AreaSlotCapacity)
				{
					break;
				}
			}
		}

		// Token: 0x06001741 RID: 5953 RVA: 0x00066430 File Offset: 0x00064630
		private void CreatePreviewAreaSlots(Dictionary<List<AreaSlot>, Area> areasBySegment, AreaSpawnBehavior spawnBehavior)
		{
			for (int i = 0; i < spawnBehavior.totalSpawnIterations; i++)
			{
				List<List<AreaSlot>> list = Enumerable.ToList<List<AreaSlot>>(areasBySegment.Keys);
				for (int j = list.Count - 1; j >= 0; j--)
				{
					Area area = areasBySegment[list[j]];
					List<AreaSlot> list2 = list[j];
					List<AreaSlot> list3 = list2;
					if (i > 0)
					{
						list3 = new List<AreaSlot>(area.EdgeAreaSlots);
					}
					if (!this.TrySpawnAreaSlotsAroundPlacedOnes(list3, area) && area.AreaSlots.Count < area.AreaSlotCapacity)
					{
						this.AddPreviewAreaToNextPreviewArea(area);
						areasBySegment.Remove(list2);
					}
				}
				Random random = new Random();
				areasBySegment = Enumerable.ToDictionary<KeyValuePair<List<AreaSlot>, Area>, List<AreaSlot>, Area>(Enumerable.OrderBy<KeyValuePair<List<AreaSlot>, Area>, int>(areasBySegment, (KeyValuePair<List<AreaSlot>, Area> x) => random.Next()), (KeyValuePair<List<AreaSlot>, Area> item) => item.Key, (KeyValuePair<List<AreaSlot>, Area> item) => item.Value);
			}
		}

		// Token: 0x06001742 RID: 5954 RVA: 0x0006653C File Offset: 0x0006473C
		private void AddPreviewAreaToNextPreviewArea(Area areaToAdd)
		{
			Debug.LogWarning(">>> TO SMALL:  " + areaToAdd.name + "! (try adding it to next preview area)");
			if (areaToAdd.Type != AreaType.Preview)
			{
				return;
			}
			Area area = null;
			foreach (AreaSlot areaSlot in areaToAdd.EdgeAreaSlots)
			{
				if (area != null)
				{
					break;
				}
				foreach (AreaSlot areaSlot2 in this.areaManager.GetAllNeighborAreaSlots(areaSlot))
				{
					if (areaSlot2 != null && areaSlot2.LocalArea != null && areaSlot2.LocalArea != areaToAdd && areaSlot2.LocalArea.Type == AreaType.Preview)
					{
						Debug.LogWarning(string.Format(">>> NEXT PREVIEW AREA FOUND! Will add {0} to {1}", areaToAdd.name, areaSlot2.LocalArea));
						area = areaSlot2.LocalArea;
						break;
					}
				}
				if (area != null)
				{
					break;
				}
			}
			this.DBG_nextAvailableArea = area;
			this.DBG_areaToAdd = areaToAdd;
			this.DBG_toSmallAreas.Add(areaToAdd);
			if (area != null)
			{
				areaToAdd.name += "_toSmall";
				foreach (AreaSlot areaSlot3 in areaToAdd.AreaSlots)
				{
					if (areaSlot3.Type != AreaType.Playable)
					{
						areaSlot3.GetComponentInChildren<Renderer>().sharedMaterial = area.previewMaterial;
						areaSlot3.LocalArea = area;
						area.AddAreaSlot(areaSlot3);
						areaToAdd.RemoveEdgeAreaSlot(areaSlot3);
						if (areaSlot3.IsLocalEdgeAreaSlot)
						{
							AreaSlot[] allNeighborAreaSlots2 = this.areaManager.GetAllNeighborAreaSlots(areaSlot3);
							areaSlot3.UpdateNeighborList(allNeighborAreaSlots2);
						}
					}
				}
				this.areaManager.ForgetLocalArea(areaToAdd);
				areaToAdd.Terminate(false, true);
				return;
			}
			Debug.LogError("No available preview area was found to which the " + areaToAdd.name + " could have been added to! \n" + string.Format("containing areaslots ({0}): {1}", areaToAdd.AreaSlots.Count, ListHelper.ListDebugString<AreaSlot>(areaToAdd.AreaSlots, ", ")));
		}

		// Token: 0x06001743 RID: 5955 RVA: 0x00066774 File Offset: 0x00064974
		private bool TrySpawnAreaSlotsAroundPlacedOnes(List<AreaSlot> placedEdgeAreaSlots, Area localAreaToPopulate)
		{
			bool flag = false;
			foreach (AreaSlot areaSlot in placedEdgeAreaSlots)
			{
				AreaSlot[] allNeighborAreaSlots = this.areaManager.GetAllNeighborAreaSlots(areaSlot);
				for (int i = 0; i < allNeighborAreaSlots.Length; i++)
				{
					if (!(allNeighborAreaSlots[i] != null))
					{
						Vector2Int? neighborGridPositionFromIndex = GridCalculator.GetNeighborGridPositionFromIndex(areaSlot.GridPos, i);
						if (neighborGridPositionFromIndex != null)
						{
							AreaSlot areaSlot2 = this.SetupAreaSlot(localAreaToPopulate, neighborGridPositionFromIndex.Value, null, false);
							flag = true;
							if (areaSlot2.LocalArea.AreaSlots.Count >= areaSlot2.LocalArea.AreaSlotCapacity)
							{
								return true;
							}
						}
					}
				}
			}
			return flag;
		}

		// Token: 0x06001744 RID: 5956 RVA: 0x00066844 File Offset: 0x00064A44
		private AreaSlot SetupAreaSlot(Area localArea, Vector2Int gridPos, Material material = null, bool isTilePlaced = false)
		{
			Vector3 vector = GridCalculator.GridToWorldPos(gridPos);
			AreaSlot areaSlot = Object.Instantiate<AreaSlot>(this.areaSlotPrefab, vector, Quaternion.identity);
			areaSlot.name = string.Format("AreaSlot {0}", areaSlot.GridPos);
			SceneOrganizer.Instance.SortInContainer(areaSlot);
			AreaType type = localArea.Type;
			Area area;
			if (type != AreaType.Playable)
			{
				if (type != AreaType.Preview)
				{
					throw new ArgumentOutOfRangeException();
				}
				area = this.areaManager.GlobalPreviewArea;
				this.areaManager.GlobalPreviewArea.AddAreaSlot(areaSlot);
			}
			else
			{
				area = this.areaManager.GlobalPlayableArea;
				this.areaManager.GlobalPlayableArea.AddAreaSlot(areaSlot);
			}
			areaSlot.InitializeAreaSlot(localArea, this.areaManager.GetAllNeighborAreaSlots(areaSlot), area, isTilePlaced);
			if (material == null)
			{
				material = areaSlot.LocalArea.previewMaterial;
			}
			areaSlot.GetComponentInChildren<Renderer>().sharedMaterial = material;
			return areaSlot;
		}

		// Token: 0x040017A5 RID: 6053
		[SerializeField]
		private AreaManager areaManager;

		// Token: 0x040017A6 RID: 6054
		[SerializeField]
		private AreaSlot areaSlotPrefab;

		// Token: 0x040017A7 RID: 6055
		[SerializeField]
		private PreviewAreaGenerator previewAreaGenerator;

		// Token: 0x040017A8 RID: 6056
		[SerializeField]
		private Area areaPrefab;

		// Token: 0x040017A9 RID: 6057
		[SerializeField]
		private AreaSpawnBehavior defaultAreaSpawnBehavior;

		// Token: 0x040017AA RID: 6058
		[SerializeField]
		internal Material defaultAreaSlotMaterial;

		// Token: 0x040017AB RID: 6059
		public Area DBG_nextAvailableArea;

		// Token: 0x040017AC RID: 6060
		public Area DBG_areaToAdd;

		// Token: 0x040017AD RID: 6061
		public List<Area> DBG_toSmallAreas;
	}
}

using System;
using System.Collections.Generic;
using System.Linq;
using UnityEngine;

namespace Dorfromantik.Area
{
	// Token: 0x020003EC RID: 1004
	public class AreaManager : MonoBehaviour
	{
		// Token: 0x170002AD RID: 685
		// (get) Token: 0x0600174D RID: 5965 RVA: 0x00066952 File Offset: 0x00064B52
		// (set) Token: 0x0600174E RID: 5966 RVA: 0x0006695A File Offset: 0x00064B5A
		internal Area GlobalPlayableArea { get; private set; }

		// Token: 0x170002AE RID: 686
		// (get) Token: 0x0600174F RID: 5967 RVA: 0x00066963 File Offset: 0x00064B63
		// (set) Token: 0x06001750 RID: 5968 RVA: 0x0006696B File Offset: 0x00064B6B
		internal Area GlobalPreviewArea { get; private set; }

		// Token: 0x170002AF RID: 687
		// (get) Token: 0x06001751 RID: 5969 RVA: 0x00066974 File Offset: 0x00064B74
		// (set) Token: 0x06001752 RID: 5970 RVA: 0x0006697C File Offset: 0x00064B7C
		internal List<Area> LocalPlayableAreas
		{
			get
			{
				return this.localPlayableAreas;
			}
			private set
			{
				this.localPlayableAreas = value;
			}
		}

		// Token: 0x170002B0 RID: 688
		// (get) Token: 0x06001753 RID: 5971 RVA: 0x00066985 File Offset: 0x00064B85
		// (set) Token: 0x06001754 RID: 5972 RVA: 0x0006698D File Offset: 0x00064B8D
		internal List<Area> LocalPreviewAreas
		{
			get
			{
				return this.localPreviewAreas;
			}
			private set
			{
				this.localPreviewAreas = value;
			}
		}

		// Token: 0x140000C8 RID: 200
		// (add) Token: 0x06001755 RID: 5973 RVA: 0x00066998 File Offset: 0x00064B98
		// (remove) Token: 0x06001756 RID: 5974 RVA: 0x000669D0 File Offset: 0x00064BD0
		public event Action<List<AreaSlot>> OnPreviewAreaPickedAsPlayable;

		// Token: 0x06001757 RID: 5975 RVA: 0x00066A08 File Offset: 0x00064C08
		private void Awake()
		{
			this.localPlayableAreas = new List<Area>();
			this.localPreviewAreas = new List<Area>();
			if (this.areaGenerator == null)
			{
				this.areaGenerator = base.GetComponent<AreaGenerator>();
			}
			if (this.previewAreaGenerator == null)
			{
				this.previewAreaGenerator = base.GetComponent<PreviewAreaGenerator>();
			}
		}

		// Token: 0x06001758 RID: 5976 RVA: 0x00066A60 File Offset: 0x00064C60
		internal AreaSlot GetAreaSlotFromGridPos(Vector2Int gridPos, AreaType areaType = AreaType.Playable, Area area = null)
		{
			Area area2 = area;
			if (area2 == null)
			{
				area2 = this.GlobalPlayableArea;
				if (areaType == AreaType.Preview)
				{
					area2 = this.GlobalPreviewArea;
				}
			}
			foreach (AreaSlot areaSlot in area2.AreaSlots)
			{
				if (areaSlot.GridPos == gridPos)
				{
					return areaSlot;
				}
			}
			return null;
		}

		// Token: 0x06001759 RID: 5977 RVA: 0x00066AE0 File Offset: 0x00064CE0
		internal void PlaceTileOnArea(Tile tile)
		{
			AreaSlot areaSlotFromGridPos = this.GetAreaSlotFromGridPos(tile.GridPos, AreaType.Playable, null);
			areaSlotFromGridPos.placedTile = tile;
			areaSlotFromGridPos.LocalArea.AddPlacedTile(tile);
			if (areaSlotFromGridPos.LocalArea.Type == AreaType.Playable)
			{
				this.GlobalPlayableArea.AddPlacedTile(tile);
			}
		}

		// Token: 0x0600175A RID: 5978 RVA: 0x00066B1C File Offset: 0x00064D1C
		internal void CreatePreviewAreas(Area completedArea)
		{
			if (completedArea != null)
			{
				completedArea.OnAreaCompletion -= new Action<Area>(this.CreatePreviewAreas);
			}
			this.ClearPreviewAreas();
			this.previewAreaGenerator.CreatePreviewAreas(this.GlobalPlayableArea, null);
			this.GlobalPlayableArea.ClearOutline();
			this.RedrawAreaOutlines();
		}

		// Token: 0x0600175B RID: 5979 RVA: 0x00066B70 File Offset: 0x00064D70
		private void ClearPreviewAreas()
		{
			List<Area> list = new List<Area>(this.LocalPreviewAreas);
			if (!Enumerable.Any<Area>(list))
			{
				return;
			}
			using (IEnumerator<List<AreaSlot>> enumerator = Enumerable.Where<List<AreaSlot>>(this.previewAreaGenerator.segmentByEdgeAreaSlot.Values, (List<AreaSlot> x) => x != null).GetEnumerator())
			{
				while (enumerator.MoveNext())
				{
					foreach (AreaSlot areaSlot in Enumerable.Where<AreaSlot>(enumerator.Current, (AreaSlot x) => x != null))
					{
						areaSlot.GetComponentInChildren<Renderer>().sharedMaterial = this.areaGenerator.defaultAreaSlotMaterial;
					}
				}
			}
			foreach (Area area in list)
			{
				area.ClearOutline();
				this.LocalPreviewAreas.Remove(area);
				area.Terminate(true, true);
			}
			this.previewAreaGenerator.TerminateAllAreaSignposts();
			this.GlobalPreviewArea.AreaSlots.Clear();
			this.GlobalPreviewArea.EdgeAreaSlots.Clear();
			this.LocalPreviewAreas.Clear();
		}

		// Token: 0x0600175C RID: 5980 RVA: 0x00066CEC File Offset: 0x00064EEC
		internal void RememberLocalArea(Area areaToRemember)
		{
			if (areaToRemember.Scope != AreaScope.Local)
			{
				return;
			}
			AreaType type = areaToRemember.Type;
			if (type == AreaType.Playable)
			{
				this.AddAreaToList(areaToRemember, ref this.localPlayableAreas);
				areaToRemember.OnAreaCompletion += new Action<Area>(this.CreatePreviewAreas);
				return;
			}
			if (type != AreaType.Preview)
			{
				return;
			}
			this.AddAreaToList(areaToRemember, ref this.localPreviewAreas);
		}

		// Token: 0x0600175D RID: 5981 RVA: 0x00066D40 File Offset: 0x00064F40
		internal void ForgetLocalArea(Area areaToForget)
		{
			if (areaToForget.Scope != AreaScope.Local)
			{
				return;
			}
			AreaType type = areaToForget.Type;
			if (type == AreaType.Playable)
			{
				this.RemoveAreaFromList(areaToForget, ref this.localPlayableAreas);
				areaToForget.OnAreaCompletion -= new Action<Area>(this.CreatePreviewAreas);
				return;
			}
			if (type != AreaType.Preview)
			{
				return;
			}
			this.RemoveAreaFromList(areaToForget, ref this.localPreviewAreas);
		}

		// Token: 0x0600175E RID: 5982 RVA: 0x00066D92 File Offset: 0x00064F92
		internal void SetupInitialAreas()
		{
			this.areaGenerator.GenerateInitialAreas();
			this.RedrawAreaOutlines();
		}

		// Token: 0x0600175F RID: 5983 RVA: 0x00066DA5 File Offset: 0x00064FA5
		internal void InitializeGlobalAreas(Area globalPlayableArea, Area globalPreviewArea)
		{
			this.GlobalPlayableArea = globalPlayableArea;
			this.GlobalPreviewArea = globalPreviewArea;
		}

		// Token: 0x06001760 RID: 5984 RVA: 0x00066DB8 File Offset: 0x00064FB8
		internal void PickPreviewAreaAsPlayable(Area area)
		{
			if (area.Type == AreaType.Playable)
			{
				return;
			}
			this.ForgetLocalArea(area);
			area.Type = AreaType.Playable;
			this.RememberLocalArea(area);
			area.name = string.Format("{0} {1} {2} #{3}", new object[]
			{
				area.Scope,
				area.Type,
				"Area",
				this.LocalPlayableAreas.Count
			});
			foreach (AreaSlot areaSlot in area.AreaSlots)
			{
				if (areaSlot.Type != AreaType.Playable)
				{
					areaSlot.Type = AreaType.Playable;
					areaSlot.IsTilePlacable = true;
					areaSlot.globalArea = this.GlobalPlayableArea;
					areaSlot.GetComponentInChildren<Renderer>().sharedMaterial = this.areaGenerator.defaultAreaSlotMaterial;
					if (!this.GlobalPlayableArea.AreaSlots.Contains(areaSlot))
					{
						this.GlobalPlayableArea.AddAreaSlot(areaSlot);
					}
				}
			}
			this.segmentOfAreaSlots = Enumerable.FirstOrDefault<KeyValuePair<List<AreaSlot>, Area>>(this.previewAreaGenerator.areasBySegment, (KeyValuePair<List<AreaSlot>, Area> x) => x.Value == area).Key;
			Action<List<AreaSlot>> onPreviewAreaPickedAsPlayable = this.OnPreviewAreaPickedAsPlayable;
			if (onPreviewAreaPickedAsPlayable != null)
			{
				onPreviewAreaPickedAsPlayable.Invoke(this.segmentOfAreaSlots);
			}
			this.ClearPreviewAreas();
			foreach (AreaSlot areaSlot2 in area.EdgeAreaSlots)
			{
				AreaSlot[] allNeighborAreaSlots = this.GetAllNeighborAreaSlots(areaSlot2);
				areaSlot2.UpdateNeighborList(allNeighborAreaSlots);
			}
			area.ClearOutline();
			this.RedrawAreaOutlines();
		}

		// Token: 0x06001761 RID: 5985 RVA: 0x00066FA8 File Offset: 0x000651A8
		internal AreaSlot[] GetAllNeighborAreaSlots(AreaSlot areaSlot)
		{
			AreaSlot[] array = new AreaSlot[6];
			Vector2Int[] array2 = GridCalculator.NeighborDirections(areaSlot.GridPos);
			for (int i = 0; i < 6; i++)
			{
				Vector2Int vector2Int = areaSlot.GridPos + array2[i];
				AreaSlot areaSlot2 = this.GetAreaSlotFromGridPos(vector2Int, AreaType.Playable, null);
				if (areaSlot2 == null)
				{
					areaSlot2 = this.GetAreaSlotFromGridPos(vector2Int, AreaType.Preview, null);
				}
				array[i] = areaSlot2;
			}
			return array;
		}

		// Token: 0x06001762 RID: 5986 RVA: 0x0006700D File Offset: 0x0006520D
		private void AddAreaToList(Area area, ref List<Area> list)
		{
			if (!list.Contains(area))
			{
				list.Add(area);
				return;
			}
			Debug.LogError(string.Format("The area ({0} - {1}) is already added to {2}!", area, area.Type, list));
		}

		// Token: 0x06001763 RID: 5987 RVA: 0x0006703F File Offset: 0x0006523F
		private void RemoveAreaFromList(Area area, ref List<Area> list)
		{
			if (list.Contains(area))
			{
				list.Remove(area);
				return;
			}
			Debug.LogError(string.Format("{0} does not contain the area ({1} - {2}), which it is trying to remove!", list, area, area.Type));
		}

		// Token: 0x06001764 RID: 5988 RVA: 0x00067074 File Offset: 0x00065274
		private void RedrawAreaOutlines()
		{
			this.GlobalPlayableArea.ClearOutline();
			if (Enumerable.Any<Area>(this.LocalPreviewAreas))
			{
				foreach (Area area in this.LocalPreviewAreas)
				{
					area.DrawOutline();
				}
				return;
			}
			this.GlobalPlayableArea.DrawOutline();
		}

		// Token: 0x040017B3 RID: 6067
		[SerializeField]
		private AreaGenerator areaGenerator;

		// Token: 0x040017B4 RID: 6068
		[SerializeField]
		private PreviewAreaGenerator previewAreaGenerator;

		// Token: 0x040017B5 RID: 6069
		[SerializeField]
		private List<Area> localPlayableAreas;

		// Token: 0x040017B6 RID: 6070
		[SerializeField]
		private List<Area> localPreviewAreas;

		// Token: 0x040017B7 RID: 6071
		private Material defaultAreaSlotMaterial;

		// Token: 0x040017BB RID: 6075
		[SerializeField]
		private List<AreaSlot> segmentOfAreaSlots;
	}
}

using System;

namespace Dorfromantik.Area
{
	// Token: 0x020003EF RID: 1007
	public enum AreaScope
	{
		// Token: 0x040017C1 RID: 6081
		Local,
		// Token: 0x040017C2 RID: 6082
		Global
	}
}

using System;
using UnityEngine;
using UnityEngine.EventSystems;

namespace Dorfromantik.Area
{
	// Token: 0x020003F0 RID: 1008
	public class AreaSignpost : MonoBehaviour, IPointerClickHandler, IEventSystemHandler
	{
		// Token: 0x0600176C RID: 5996 RVA: 0x0006710E File Offset: 0x0006530E
		internal void Initialize(Area area, AreaManager areaManager)
		{
			this.area = area;
			this.areaManager = areaManager;
		}

		// Token: 0x0600176D RID: 5997 RVA: 0x0006711E File Offset: 0x0006531E
		public void OnPointerClick(PointerEventData eventData)
		{
			this.areaManager.PickPreviewAreaAsPlayable(this.area);
		}

		// Token: 0x0600176E RID: 5998 RVA: 0x0002EB53 File Offset: 0x0002CD53
		internal void Terminate()
		{
			Object.Destroy(base.gameObject);
		}

		// Token: 0x040017C3 RID: 6083
		private Area area;

		// Token: 0x040017C4 RID: 6084
		private AreaManager areaManager;
	}
}

using System;
using System.Collections.Generic;
using System.Linq;
using UnityEngine;

namespace Dorfromantik.Area
{
	// Token: 0x020003F1 RID: 1009
	public class AreaSlot : MonoBehaviour, IOutlineable
	{
		// Token: 0x170002B1 RID: 689
		// (get) Token: 0x06001770 RID: 6000 RVA: 0x00067131 File Offset: 0x00065331
		// (set) Token: 0x06001771 RID: 6001 RVA: 0x00067139 File Offset: 0x00065339
		internal AreaSlot[] AllNeighbors { get; private set; }

		// Token: 0x170002B2 RID: 690
		// (get) Token: 0x06001772 RID: 6002 RVA: 0x00067142 File Offset: 0x00065342
		// (set) Token: 0x06001773 RID: 6003 RVA: 0x0006714A File Offset: 0x0006534A
		public AreaSlot[] NeighborsInLocalArea { get; private set; }

		// Token: 0x170002B3 RID: 691
		// (get) Token: 0x06001774 RID: 6004 RVA: 0x00067153 File Offset: 0x00065353
		// (set) Token: 0x06001775 RID: 6005 RVA: 0x0006715B File Offset: 0x0006535B
		internal AreaSlot[] NeighborsInGlobalArea { get; private set; }

		// Token: 0x170002B4 RID: 692
		// (get) Token: 0x06001776 RID: 6006 RVA: 0x00067164 File Offset: 0x00065364
		// (set) Token: 0x06001777 RID: 6007 RVA: 0x0006716C File Offset: 0x0006536C
		internal Area LocalArea { get; set; }

		// Token: 0x170002B5 RID: 693
		// (get) Token: 0x06001778 RID: 6008 RVA: 0x00067175 File Offset: 0x00065375
		// (set) Token: 0x06001779 RID: 6009 RVA: 0x0006717D File Offset: 0x0006537D
		internal bool IsTilePlacable { get; set; }

		// Token: 0x170002B6 RID: 694
		// (get) Token: 0x0600177A RID: 6010 RVA: 0x00067186 File Offset: 0x00065386
		// (set) Token: 0x0600177B RID: 6011 RVA: 0x0006718E File Offset: 0x0006538E
		internal Vector2Int GridPos
		{
			get
			{
				return this.gridPos;
			}
			private set
			{
				this.gridPos = value;
				base.transform.position = GridCalculator.GridToWorldPos(value);
			}
		}

		// Token: 0x170002B7 RID: 695
		// (get) Token: 0x0600177C RID: 6012 RVA: 0x000671A8 File Offset: 0x000653A8
		// (set) Token: 0x0600177D RID: 6013 RVA: 0x000671B0 File Offset: 0x000653B0
		internal bool IsLocalEdgeAreaSlot
		{
			get
			{
				return this.isLocalEdgeAreaSlot;
			}
			private set
			{
				this.isLocalEdgeAreaSlot = value;
				if (value)
				{
					this.LocalArea.AddEdgeAreaSlot(this);
					return;
				}
				this.LocalArea.RemoveEdgeAreaSlot(this);
			}
		}

		// Token: 0x170002B8 RID: 696
		// (get) Token: 0x0600177E RID: 6014 RVA: 0x000671D5 File Offset: 0x000653D5
		// (set) Token: 0x0600177F RID: 6015 RVA: 0x000671DD File Offset: 0x000653DD
		internal bool IsGlobalEdgeAreaSlot
		{
			get
			{
				return this.isGlobalEdgeAreaSlot;
			}
			private set
			{
				this.isGlobalEdgeAreaSlot = value;
				if (value)
				{
					this.globalArea.AddEdgeAreaSlot(this);
					return;
				}
				this.globalArea.RemoveEdgeAreaSlot(this);
			}
		}

		// Token: 0x170002B9 RID: 697
		// (get) Token: 0x06001780 RID: 6016 RVA: 0x00067202 File Offset: 0x00065402
		// (set) Token: 0x06001781 RID: 6017 RVA: 0x0006720A File Offset: 0x0006540A
		internal AreaType Type { get; set; }

		// Token: 0x170002BA RID: 698
		// (get) Token: 0x06001782 RID: 6018 RVA: 0x00067214 File Offset: 0x00065414
		public IOutlineable[] Neighbors
		{
			get
			{
				return this.GetNeighborsBasedOnLocalAreaType();
			}
		}

		// Token: 0x06001783 RID: 6019 RVA: 0x00067229 File Offset: 0x00065429
		IOutlineable IOutlineable.GetNeighbor(int edgeIndex, Space space)
		{
			return this.GetNeighborsBasedOnLocalAreaType()[edgeIndex];
		}

		// Token: 0x170002BB RID: 699
		// (get) Token: 0x06001784 RID: 6020 RVA: 0x0002DA03 File Offset: 0x0002BC03
		public Vector3 WorldPosition
		{
			get
			{
				return base.transform.position;
			}
		}

		// Token: 0x06001785 RID: 6021 RVA: 0x00067233 File Offset: 0x00065433
		private void Awake()
		{
			this.AllNeighbors = new AreaSlot[6];
			this.NeighborsInLocalArea = new AreaSlot[6];
			this.NeighborsInGlobalArea = new AreaSlot[6];
			this.GridPos = GridCalculator.WorldToGridPos(base.transform.position);
		}

		// Token: 0x06001786 RID: 6022 RVA: 0x0006726F File Offset: 0x0006546F
		private void OnDestroy()
		{
			this.UpdateAreaSlotNeighborsNeighborList(false);
		}

		// Token: 0x06001787 RID: 6023 RVA: 0x00067278 File Offset: 0x00065478
		internal void InitializeAreaSlot(Area areaLocal, AreaSlot[] areaSlotNeighbors, Area areaGlobal, bool isTilePlaced = false)
		{
			this.LocalArea = areaLocal;
			this.LocalArea.AddAreaSlot(this);
			this.globalArea = areaGlobal;
			this.Type = areaLocal.Type;
			this.IsTilePlacable = isTilePlaced;
			this.UpdateNeighborList(areaSlotNeighbors);
			if (areaLocal.Type == AreaType.Playable)
			{
				this.IsTilePlacable = true;
			}
		}

		// Token: 0x06001788 RID: 6024 RVA: 0x000672CC File Offset: 0x000654CC
		internal void UpdateNeighborList(AreaSlot[] areaSlotNeighbors)
		{
			for (int i = 0; i < areaSlotNeighbors.Length; i++)
			{
				this.AddNeighbor(areaSlotNeighbors[i], i);
			}
			this.UpdateAreaSlotNeighborsNeighborList(false);
		}

		// Token: 0x06001789 RID: 6025 RVA: 0x000672F8 File Offset: 0x000654F8
		internal void UpdateAreaSlotNeighborsNeighborList(bool shouldRemoveItself = false)
		{
			foreach (AreaSlot areaSlot in this.AllNeighbors)
			{
				if (!(areaSlot == null))
				{
					int? neighborIndexFromGridPos = GridCalculator.GetNeighborIndexFromGridPos(areaSlot.GridPos, this.GridPos);
					if (neighborIndexFromGridPos != null)
					{
						areaSlot.AddNeighbor(shouldRemoveItself ? null : this, neighborIndexFromGridPos.Value);
					}
				}
			}
		}

		// Token: 0x0600178A RID: 6026 RVA: 0x00067358 File Offset: 0x00065558
		private void AddNeighbor(AreaSlot neighbor, int neighborIndex)
		{
			if (neighborIndex < 0 || neighborIndex > 5)
			{
				Debug.LogError(string.Format("Passed neighbor index ({0}) is not a valid neighbor index (should be [0...5]! {1} was not added as neighbor to {2}", neighborIndex, neighbor, this));
				return;
			}
			try
			{
				this.AllNeighbors[neighborIndex] = neighbor;
				if (neighbor == null)
				{
					this.NeighborsInGlobalArea[neighborIndex] = neighbor;
					this.NeighborsInLocalArea[neighborIndex] = neighbor;
				}
				else if (neighbor.LocalArea.Type == this.LocalArea.Type)
				{
					this.NeighborsInGlobalArea[neighborIndex] = neighbor;
					if (!(neighbor.LocalArea != this.LocalArea))
					{
						this.NeighborsInLocalArea[neighborIndex] = neighbor;
					}
				}
			}
			finally
			{
				this.CheckIfIsEdgeAreaSlot();
			}
		}

		// Token: 0x0600178B RID: 6027 RVA: 0x00067408 File Offset: 0x00065608
		internal void CheckIfIsEdgeAreaSlot()
		{
			this.IsGlobalEdgeAreaSlot = Enumerable.Any<AreaSlot>(this.NeighborsInGlobalArea, (AreaSlot neighborGlobalArea) => neighborGlobalArea == null);
			this.IsLocalEdgeAreaSlot = Enumerable.Any<AreaSlot>(this.NeighborsInLocalArea, (AreaSlot neighborLocalArea) => neighborLocalArea == null);
		}

		// Token: 0x0600178C RID: 6028 RVA: 0x00067478 File Offset: 0x00065678
		internal void UpdateLocalAndGlobalNeighborsFromAllNeighbors()
		{
			this.NeighborsInGlobalArea = this.AllNeighbors;
			for (int i = 0; i < this.NeighborsInGlobalArea.Length; i++)
			{
				if (!(this.NeighborsInGlobalArea[i] == null) && this.NeighborsInGlobalArea[i].LocalArea.Type != this.LocalArea.Type)
				{
					this.NeighborsInGlobalArea[i] = null;
				}
			}
			this.NeighborsInLocalArea = this.NeighborsInGlobalArea;
			for (int j = 0; j < this.NeighborsInLocalArea.Length; j++)
			{
				if (!(this.NeighborsInLocalArea[j] == null) && this.NeighborsInLocalArea[j].LocalArea != this.LocalArea)
				{
					this.NeighborsInLocalArea[j] = null;
				}
			}
		}

		// Token: 0x0600178D RID: 6029 RVA: 0x00067530 File Offset: 0x00065730
		private AreaSlot[] GetNeighborsBasedOnLocalAreaType()
		{
			AreaType type = this.LocalArea.Type;
			if (type == AreaType.Playable)
			{
				return this.NeighborsInGlobalArea;
			}
			if (type != AreaType.Preview)
			{
				throw new ArgumentOutOfRangeException();
			}
			return this.NeighborsInLocalArea;
		}

		// Token: 0x040017C5 RID: 6085
		public string didCheckit;

		// Token: 0x040017C6 RID: 6086
		public List<AreaSlot> passedNeighbors;

		// Token: 0x040017C7 RID: 6087
		[SerializeField]
		private Vector2Int gridPos;

		// Token: 0x040017CB RID: 6091
		[SerializeField]
		private bool isLocalEdgeAreaSlot;

		// Token: 0x040017CC RID: 6092
		[SerializeField]
		private bool isGlobalEdgeAreaSlot;

		// Token: 0x040017CD RID: 6093
		[SerializeField]
		internal Tile placedTile;

		// Token: 0x040017CE RID: 6094
		[SerializeField]
		internal Area globalArea;
	}
}

using System;
using UnityEngine;

namespace Dorfromantik.Area
{
	// Token: 0x020003F3 RID: 1011
	public class AreaSpawnBehavior : ScriptableObject
	{
		// Token: 0x040017D5 RID: 6101
		[SerializeField]
		internal Vector2Int tilesCountMinMax = new Vector2Int(0, 0);

		// Token: 0x040017D6 RID: 6102
		[SerializeField]
		internal Vector2Int edgeAreaSlotSegmentCountMinMax = new Vector2Int(0, 0);

		// Token: 0x040017D7 RID: 6103
		[SerializeField]
		internal int totalSpawnIterations;

		// Token: 0x040017D8 RID: 6104
		[SerializeField]
		internal int completionPercentageNeeded;
	}
}

using System;

namespace Dorfromantik.Area
{
	// Token: 0x020003F4 RID: 1012
	public enum AreaType
	{
		// Token: 0x040017DA RID: 6106
		Playable,
		// Token: 0x040017DB RID: 6107
		Preview
	}
}

using System;
using System.Collections.Generic;
using System.Linq;
using UnityEngine;

namespace Dorfromantik.Area
{
	// Token: 0x020003F5 RID: 1013
	public class PreviewAreaGenerator : MonoBehaviour
	{
		// Token: 0x06001794 RID: 6036 RVA: 0x00067593 File Offset: 0x00065793
		public void Awake()
		{
			if (this.areaGenerator == null)
			{
				this.areaGenerator = base.GetComponent<AreaGenerator>();
			}
			if (this.areaManager == null)
			{
				this.areaManager = base.GetComponent<AreaManager>();
			}
		}

		// Token: 0x06001795 RID: 6037 RVA: 0x000675CC File Offset: 0x000657CC
		internal List<Area> CreatePreviewAreas(Area area, AreaSpawnBehavior spawnBehavior = null)
		{
			this.spawnBehavior = ((spawnBehavior == null) ? this.defaultPreviewSpawnBehavior : spawnBehavior);
			List<List<AreaSlot>> list = this.SplitEdgeAreaSlotsIntoSegments(area, null);
			this.areasBySegment = this.areaGenerator.CreatePreviewAreas(this.spawnBehavior, list);
			this.SetupAreaSignposts();
			return Enumerable.ToList<Area>(this.areasBySegment.Values);
		}

		// Token: 0x06001796 RID: 6038 RVA: 0x00067628 File Offset: 0x00065828
		private List<List<AreaSlot>> SplitEdgeAreaSlotsIntoSegments(Area area, AreaSlot initialEdgeAreaSlot = null)
		{
			this.segmentByEdgeAreaSlot = Enumerable.ToDictionary<AreaSlot, AreaSlot, List<AreaSlot>>(area.EdgeAreaSlots, (AreaSlot areaSlot) => areaSlot, (AreaSlot segment) => null);
			this.initialEdgeAreaSlots.Clear();
			this.leftoverAreaSlots.Clear();
			List<List<AreaSlot>> list = new List<List<AreaSlot>>();
			int x2 = this.spawnBehavior.edgeAreaSlotSegmentCountMinMax.x;
			int y = this.spawnBehavior.edgeAreaSlotSegmentCountMinMax.y;
			if (initialEdgeAreaSlot == null)
			{
				initialEdgeAreaSlot = area.EdgeAreaSlots[Random.Range(0, area.EdgeAreaSlots.Count - 1)];
			}
			this.initialEdgeAreaSlots.Add(initialEdgeAreaSlot);
			int num = 0;
			while (this.GetTotalCountOfEdgeAreaSlotsInSegment(list) < area.EdgeAreaSlots.Count && num < 100)
			{
				num++;
				int num2 = Random.Range(x2, y);
				int num3 = area.EdgeAreaSlots.Count - this.GetTotalCountOfEdgeAreaSlotsInSegment(list);
				if (num3 < x2)
				{
					List<AreaSlot> list2 = Enumerable.ToList<AreaSlot>(Enumerable.Select<KeyValuePair<AreaSlot, List<AreaSlot>>, AreaSlot>(Enumerable.Where<KeyValuePair<AreaSlot, List<AreaSlot>>>(this.segmentByEdgeAreaSlot, (KeyValuePair<AreaSlot, List<AreaSlot>> x) => x.Value == null), (KeyValuePair<AreaSlot, List<AreaSlot>> pair) => pair.Key));
					this.AddEdgeAreaSlotsToNearestSegment(list2);
				}
				else
				{
					if (num3 < num2)
					{
						num2 = num3;
					}
					List<AreaSlot> list3 = this.CreateNewSegment(num2);
					if (list3 != null)
					{
						list.Add(list3);
					}
					if (this.initialEdgeAreaSlots.Count == 0)
					{
						AreaSlot randomAvailableInitialEdgeAreaSlot = this.GetRandomAvailableInitialEdgeAreaSlot(list);
						if (!(randomAvailableInitialEdgeAreaSlot != null))
						{
							break;
						}
						this.initialEdgeAreaSlots.Add(randomAvailableInitialEdgeAreaSlot);
					}
				}
			}
			return list;
		}

		// Token: 0x06001797 RID: 6039 RVA: 0x000677F0 File Offset: 0x000659F0
		private List<AreaSlot> CreateNewSegment(int segmentSize)
		{
			List<AreaSlot> finalEdgeAreaSlotSegment = new List<AreaSlot>();
			List<AreaSlot> list = new List<AreaSlot>(this.initialEdgeAreaSlots);
			Predicate<AreaSlot> <>9__0;
			foreach (AreaSlot areaSlot in this.initialEdgeAreaSlots)
			{
				List<AreaSlot> list2 = new List<AreaSlot>();
				list2.Add(areaSlot);
				List<AreaSlot> list3 = list2;
				finalEdgeAreaSlotSegment = new List<AreaSlot>(list3);
				int num = 1;
				while (finalEdgeAreaSlotSegment.Count < segmentSize + 1 && num < 100)
				{
					num++;
					list3 = new List<AreaSlot>(finalEdgeAreaSlotSegment);
					foreach (AreaSlot areaSlot2 in list3)
					{
						if (this.segmentByEdgeAreaSlot[areaSlot2] == null)
						{
							foreach (AreaSlot areaSlot3 in Enumerable.Where<AreaSlot>(Enumerable.ToList<AreaSlot>(this.GetAllAvailableEdgeAreaSlotNeighbors(areaSlot2)), (AreaSlot x) => x != null))
							{
								if (finalEdgeAreaSlotSegment.Count < segmentSize)
								{
									finalEdgeAreaSlotSegment.Add(areaSlot3);
									if (this.segmentByEdgeAreaSlot.ContainsKey(areaSlot2))
									{
										this.segmentByEdgeAreaSlot[areaSlot2] = finalEdgeAreaSlotSegment;
									}
									if (this.leftoverAreaSlots.Contains(areaSlot3))
									{
										this.leftoverAreaSlots.Remove(areaSlot3);
									}
									if (finalEdgeAreaSlotSegment.Count >= segmentSize)
									{
										break;
									}
								}
								else
								{
									list.Add(areaSlot3);
								}
							}
							if (finalEdgeAreaSlotSegment.Count >= segmentSize)
							{
								break;
							}
						}
					}
					if (list3.Count == finalEdgeAreaSlotSegment.Count)
					{
						break;
					}
				}
				if (finalEdgeAreaSlotSegment.Count != segmentSize)
				{
					if (finalEdgeAreaSlotSegment.Count > segmentSize)
					{
						Debug.LogError(string.Format("The amount of area slots in this segment ({0}) should never be more than the predefined segment size ({1})!", finalEdgeAreaSlotSegment.Count, segmentSize));
					}
					this.leftoverAreaSlots.AddRange(finalEdgeAreaSlotSegment);
					this.leftoverAreaSlots = Enumerable.ToList<AreaSlot>(Enumerable.Distinct<AreaSlot>(this.leftoverAreaSlots));
					List<AreaSlot> list4 = list;
					Predicate<AreaSlot> predicate;
					if ((predicate = <>9__0) == null)
					{
						predicate = (<>9__0 = (AreaSlot x) => Enumerable.Any<AreaSlot>(finalEdgeAreaSlotSegment, (AreaSlot y) => y == x));
					}
					list4.RemoveAll(predicate);
					finalEdgeAreaSlotSegment = null;
				}
				else
				{
					foreach (AreaSlot areaSlot4 in finalEdgeAreaSlotSegment)
					{
						this.segmentByEdgeAreaSlot[areaSlot4] = finalEdgeAreaSlotSegment;
						if (list.Contains(areaSlot4))
						{
							list.Remove(areaSlot4);
						}
					}
				}
			}
			this.initialEdgeAreaSlots = list;
			return finalEdgeAreaSlotSegment;
		}

		// Token: 0x06001798 RID: 6040 RVA: 0x00067B3C File Offset: 0x00065D3C
		private void AddEdgeAreaSlotsToNearestSegment(List<AreaSlot> edgeAreaSlots)
		{
			foreach (AreaSlot areaSlot in edgeAreaSlots)
			{
				foreach (AreaSlot areaSlot2 in this.GetAllEdgeAreaSlotNeighbors(areaSlot))
				{
					if (!(areaSlot2 == null) && this.segmentByEdgeAreaSlot[areaSlot2] != null)
					{
						this.segmentByEdgeAreaSlot[areaSlot] = this.segmentByEdgeAreaSlot[areaSlot2];
						this.segmentByEdgeAreaSlot[areaSlot2].Add(areaSlot);
					}
				}
			}
		}

		// Token: 0x06001799 RID: 6041 RVA: 0x00067BE4 File Offset: 0x00065DE4
		private AreaSlot[] GetAllEdgeAreaSlotNeighbors(AreaSlot areaSlot)
		{
			AreaSlot[] array = new AreaSlot[6];
			areaSlot.NeighborsInLocalArea.CopyTo(array, 0);
			for (int i = 0; i < array.Length; i++)
			{
				if (!(array[i] == null) && (!array[i].IsLocalEdgeAreaSlot || !this.segmentByEdgeAreaSlot.ContainsKey(array[i])))
				{
					array[i] = null;
				}
			}
			return array;
		}

		// Token: 0x0600179A RID: 6042 RVA: 0x00067C40 File Offset: 0x00065E40
		private AreaSlot[] GetAllAvailableEdgeAreaSlotNeighbors(AreaSlot areaSlot)
		{
			AreaSlot[] allEdgeAreaSlotNeighbors = this.GetAllEdgeAreaSlotNeighbors(areaSlot);
			for (int i = 0; i < allEdgeAreaSlotNeighbors.Length; i++)
			{
				if (!(allEdgeAreaSlotNeighbors[i] == null) && (this.segmentByEdgeAreaSlot[allEdgeAreaSlotNeighbors[i]] != null || this.leftoverAreaSlots.Contains(allEdgeAreaSlotNeighbors[i])))
				{
					allEdgeAreaSlotNeighbors[i] = null;
				}
			}
			return allEdgeAreaSlotNeighbors;
		}

		// Token: 0x0600179B RID: 6043 RVA: 0x00067C94 File Offset: 0x00065E94
		private AreaSlot GetRandomAvailableInitialEdgeAreaSlot(List<List<AreaSlot>> segmentOfAreaSlots)
		{
			foreach (KeyValuePair<AreaSlot, List<AreaSlot>> keyValuePair in this.segmentByEdgeAreaSlot)
			{
				if (keyValuePair.Value == null && !this.leftoverAreaSlots.Contains(keyValuePair.Key))
				{
					return keyValuePair.Key;
				}
			}
			return null;
		}

		// Token: 0x0600179C RID: 6044 RVA: 0x00067D0C File Offset: 0x00065F0C
		private int GetTotalCountOfEdgeAreaSlotsInSegment(List<List<AreaSlot>> segmentOfAreaSlots)
		{
			return Enumerable.Sum<List<AreaSlot>>(segmentOfAreaSlots, (List<AreaSlot> segment) => segment.Count);
		}

		// Token: 0x0600179D RID: 6045 RVA: 0x00067D34 File Offset: 0x00065F34
		private void SetupAreaSignposts()
		{
			foreach (KeyValuePair<List<AreaSlot>, Area> keyValuePair in this.areasBySegment)
			{
				Vector3 vector = GridCalculator.GridToWorldPos(keyValuePair.Key[Random.Range(0, Enumerable.Count<AreaSlot>(keyValuePair.Key))].GridPos);
				AreaSignpost areaSignpost = Object.Instantiate<AreaSignpost>(this.areaSignpostPrefab, vector, Quaternion.identity);
				areaSignpost.name = "AreaSignpost - " + keyValuePair.Value.name;
				areaSignpost.Initialize(keyValuePair.Value, this.areaManager);
				areaSignpost.GetComponentInChildren<Renderer>().sharedMaterial = keyValuePair.Value.previewMaterial;
				this.areasignpostsByArea.Add(keyValuePair.Value, areaSignpost);
				keyValuePair.Value.areaSignpost = areaSignpost;
			}
		}

		// Token: 0x0600179E RID: 6046 RVA: 0x00067E28 File Offset: 0x00066028
		internal void TerminateAllAreaSignposts()
		{
			AreaSignpost[] array = Object.FindObjectsOfType<AreaSignpost>();
			if (!Enumerable.Any<AreaSignpost>(array))
			{
				return;
			}
			AreaSignpost[] array2 = array;
			for (int i = 0; i < array2.Length; i++)
			{
				array2[i].Terminate();
			}
			this.areasignpostsByArea.Clear();
		}

		// Token: 0x040017DC RID: 6108
		[SerializeField]
		private AreaSpawnBehavior defaultPreviewSpawnBehavior;

		// Token: 0x040017DD RID: 6109
		[SerializeField]
		private AreaManager areaManager;

		// Token: 0x040017DE RID: 6110
		[SerializeField]
		private AreaGenerator areaGenerator;

		// Token: 0x040017DF RID: 6111
		[SerializeField]
		private AreaSignpost areaSignpostPrefab;

		// Token: 0x040017E0 RID: 6112
		private AreaSpawnBehavior spawnBehavior;

		// Token: 0x040017E1 RID: 6113
		private List<AreaSlot> initialEdgeAreaSlots = new List<AreaSlot>();

		// Token: 0x040017E2 RID: 6114
		private List<AreaSlot> leftoverAreaSlots = new List<AreaSlot>();

		// Token: 0x040017E3 RID: 6115
		internal Dictionary<List<AreaSlot>, Area> areasBySegment = new Dictionary<List<AreaSlot>, Area>();

		// Token: 0x040017E4 RID: 6116
		internal Dictionary<Area, AreaSignpost> areasignpostsByArea = new Dictionary<Area, AreaSignpost>();

		// Token: 0x040017E5 RID: 6117
		internal Dictionary<AreaSlot, List<AreaSlot>> segmentByEdgeAreaSlot = new Dictionary<AreaSlot, List<AreaSlot>>();
	}
}
