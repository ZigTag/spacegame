using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class TargetableObject : MonoBehaviour
{
    public bool targeted;
    public GameObject outlinePrefab;
    public GameObject outlineInnerPrefab;
    GameObject outlinePrefabSet;
    GameObject outlineInnerPrefabSet;
    Vector3 prefOutlineScale;
    Vector3 prefOutlineInnerScale;

    void Start() {
        var outlineScale = .20f;
        prefOutlineScale = transform.lossyScale + new Vector3(outlineScale, outlineScale, 0);
        prefOutlineInnerScale = transform.lossyScale + new Vector3(outlineScale / 1.5f, outlineScale / 1.5f, 0);
    }

    void Update()
    {
        if (targeted) {
            if (outlinePrefabSet == null) {
                GameObject outline = Instantiate(outlinePrefab);
                outline.transform.localScale = prefOutlineScale;
                outline.name = "Outline";
                outlinePrefabSet = outline;
            }
            if (outlineInnerPrefabSet == null) {
                GameObject outlineInner = Instantiate(outlineInnerPrefab);
                outlineInner.transform.localScale = prefOutlineInnerScale;
                outlineInner.name = "Outline Inner";
                outlineInnerPrefabSet = outlineInner;
            }
            
            outlinePrefabSet.transform.position = transform.position + new Vector3(0, 0, 2);
            outlineInnerPrefabSet.transform.position = transform.position + new Vector3(0, 0, 1);
        } else {
            if (outlinePrefabSet != null) {
                GameObject.Destroy(outlinePrefabSet);
                outlinePrefabSet = null;
            }
            if (outlineInnerPrefabSet != null) {
                GameObject.Destroy(outlineInnerPrefabSet);
                outlineInnerPrefabSet = null;
            }
        }
    }

    // Activates when mouse is hovered over box collider
    void OnMouseOver()
    {
        if (Input.GetMouseButtonDown(0))
        {
            TargetableObject[] targets = FindObjectsOfType<TargetableObject>();
            print(targets.Length);

            for (int i = 0; i < targets.Length; i++)
            {   
                if (targets[i].targeted) {
                    targets[i].targeted = false;
                }
            }

            targeted = true;
        }
    }
}
