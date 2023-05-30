using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class BitcraftMiniGameManager : MonoBehaviour
{
    public static BitcraftMiniGameManager instance;

    public GameObject PlayerPrefab;

    [SerializeField] private GameObject preSpawnCamera;

    // Start is called before the first frame update
    void Start()
    {
        instance = this;

        UIUsernameChooser.instance.Show();
    }

    public void StartGame()
    {
        preSpawnCamera.SetActive(false);
        UIChatController.instance.Show();
    }
}
